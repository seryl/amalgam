//! Special Case Registry - Data-driven handling of edge cases and exceptions
//!
//! This module provides a clean, declarative way to handle special cases
//! without cluttering the main pipeline code. Special cases are defined
//! as data structures that can be composed and pipelined.

mod pipeline;

pub use pipeline::{SpecialCasePipeline, WithSpecialCases};

use crate::naming::to_camel_case as naming_to_camel_case;
use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry of all special cases in the system
/// This acts as a central repository for edge cases, keeping them
/// out of the main pipeline code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialCaseRegistry {
    /// Type name transformations (e.g., io.k8s.api.core.v1.ObjectMeta -> ObjectMeta)
    #[serde(default)]
    type_transforms: Vec<TypeTransform>,

    /// Module path remappings (e.g., io.k8s -> k8s.io)
    #[serde(default)]
    module_remappings: Vec<ModuleRemapping>,

    /// Import path overrides for specific type combinations
    #[serde(default)]
    import_overrides: Vec<ImportOverride>,

    /// Field naming exceptions (e.g., $ref -> ref_field)
    #[serde(default)]
    field_renames: Vec<FieldRename>,

    /// Type coercion hints (e.g., IntOrString prefers string)
    #[serde(default)]
    type_coercions: Vec<TypeCoercion>,
}

/// A transformation rule for type names
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeTransform {
    /// Pattern to match (can use wildcards)
    pub pattern: String,
    /// Context where this applies (e.g., "openapi", "crd", "*")
    pub context: String,
    /// Transformation to apply
    pub transform: TransformAction,
}

/// Actions that can be applied to transform types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformAction {
    /// Remove a prefix
    RemovePrefix(String),
    /// Remove a suffix
    RemoveSuffix(String),
    /// Replace a pattern
    Replace { from: String, to: String },
    /// Apply a function by name
    Function(String),
}

/// Module path remapping rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleRemapping {
    /// Original module pattern (e.g., "io.k8s.*")
    pub from_pattern: String,
    /// Target module pattern (e.g., "k8s.io.$1")
    pub to_pattern: String,
    /// Priority for overlapping rules (higher wins)
    pub priority: i32,
}

/// Import path override for specific cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOverride {
    /// Source module
    pub from_module: String,
    /// Target type
    pub target_type: String,
    /// Override import path
    pub import_path: String,
    /// Reason for override (for documentation)
    pub reason: String,
}

/// Field renaming rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldRename {
    /// Type containing the field (can use wildcards)
    pub type_pattern: String,
    /// Original field name
    pub from_field: String,
    /// New field name
    pub to_field: String,
}

/// Type coercion hint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCoercion {
    /// Type name pattern
    pub type_pattern: String,
    /// Coercion strategy
    pub strategy: CoercionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoercionStrategy {
    PreferString,
    PreferNumber,
    PreferFirst,
    Custom(String),
}

/// Trait for applying special case rules in a pipeline
pub trait SpecialCaseHandler {
    /// Apply this handler to a type
    fn apply_to_type(&self, ty: &Type, context: &Context) -> Option<Type>;

    /// Apply this handler to a module path
    fn apply_to_module(&self, module: &str, context: &Context) -> Option<String>;

    /// Check if this handler applies to the given context
    fn matches_context(&self, context: &Context) -> bool;
}

/// Context for applying special cases
#[derive(Debug, Clone)]
pub struct Context {
    /// Current module being processed
    pub current_module: String,
    /// Source type (openapi, crd, go, etc.)
    pub source_type: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl SpecialCaseRegistry {
    /// Load special cases from a configuration file
    pub fn from_config(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let registry: SpecialCaseRegistry = toml::from_str(&content)?;
        Ok(registry)
    }

    /// Load from embedded configuration files
    pub fn new() -> Self {
        // Try to load from embedded TOML files
        let mut registry = SpecialCaseRegistry {
            type_transforms: vec![],
            module_remappings: vec![],
            import_overrides: vec![],
            field_renames: vec![],
            type_coercions: vec![],
        };

        // Load k8s rules
        if let Ok(k8s_rules) = Self::load_embedded_rules(include_str!("rules/k8s.toml")) {
            registry.merge(k8s_rules);
        }

        // Load common rules
        if let Ok(common_rules) = Self::load_embedded_rules(include_str!("rules/common.toml")) {
            registry.merge(common_rules);
        }

        // Load crossplane rules
        if let Ok(crossplane_rules) =
            Self::load_embedded_rules(include_str!("rules/crossplane.toml"))
        {
            registry.merge(crossplane_rules);
        }

        registry
    }

    /// Load rules from an embedded string
    fn load_embedded_rules(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let registry: SpecialCaseRegistry = toml::from_str(content)?;
        Ok(registry)
    }

    /// Merge another registry into this one
    pub fn merge(&mut self, other: SpecialCaseRegistry) {
        self.type_transforms.extend(other.type_transforms);
        self.module_remappings.extend(other.module_remappings);
        self.import_overrides.extend(other.import_overrides);
        self.field_renames.extend(other.field_renames);
        self.type_coercions.extend(other.type_coercions);
    }

    /// Apply all relevant transformations to a type name
    pub fn transform_type_name(&self, name: &str, context: &Context) -> String {
        let mut result = name.to_string();

        for transform in &self.type_transforms {
            if self.matches_pattern(&transform.pattern, name)
                && (transform.context == "*" || transform.context == context.source_type)
            {
                result = self.apply_transform_action(&result, &transform.transform);
            }
        }

        result
    }

    /// Remap a module path according to rules
    pub fn remap_module(&self, module: &str) -> String {
        let mut candidates = vec![];

        for remapping in &self.module_remappings {
            if let Some(remapped) = self.apply_remapping(module, remapping) {
                candidates.push((remapping.priority, remapped));
            }
        }

        // Sort by priority (highest first) and take the first
        candidates.sort_by(|a, b| b.0.cmp(&a.0));
        candidates
            .into_iter()
            .next()
            .map(|(_, remapped)| remapped)
            .unwrap_or_else(|| module.to_string())
    }

    /// Get import override if one exists
    pub fn get_import_override(
        &self,
        from_module: &str,
        target_type: &str,
    ) -> Option<&ImportOverride> {
        self.import_overrides
            .iter()
            .find(|o| o.from_module == from_module && o.target_type == target_type)
    }

    /// Check if a field should be renamed
    pub fn get_field_rename(&self, type_name: &str, field_name: &str) -> Option<String> {
        for rename in &self.field_renames {
            if self.matches_pattern(&rename.type_pattern, type_name)
                && rename.from_field == field_name
            {
                return Some(rename.to_field.clone());
            }
        }
        None
    }

    /// Get type coercion strategy
    pub fn get_coercion_strategy(&self, type_name: &str) -> Option<&CoercionStrategy> {
        self.type_coercions
            .iter()
            .find(|c| self.matches_pattern(&c.type_pattern, type_name))
            .map(|c| &c.strategy)
    }

    // Helper methods

    fn matches_pattern(&self, pattern: &str, text: &str) -> bool {
        // Simple wildcard matching (can be enhanced with regex)
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with('*') && pattern.ends_with('*') {
            let middle = &pattern[1..pattern.len() - 1];
            return text.contains(middle);
        }

        if let Some(suffix) = pattern.strip_prefix('*') {
            return text.ends_with(suffix);
        }

        if let Some(prefix) = pattern.strip_suffix('*') {
            return text.starts_with(prefix);
        }

        pattern == text
    }

    fn apply_transform_action(&self, text: &str, action: &TransformAction) -> String {
        match action {
            TransformAction::RemovePrefix(prefix) => {
                text.strip_prefix(prefix).unwrap_or(text).to_string()
            }
            TransformAction::RemoveSuffix(suffix) => {
                text.strip_suffix(suffix).unwrap_or(text).to_string()
            }
            TransformAction::Replace { from, to } => text.replace(from, to),
            TransformAction::Function(func_name) => {
                // Could dispatch to registered functions
                match func_name.as_str() {
                    "to_camel_case" => naming_to_camel_case(text),
                    _ => text.to_string(),
                }
            }
        }
    }

    fn apply_remapping(&self, module: &str, remapping: &ModuleRemapping) -> Option<String> {
        // Simple pattern matching with capture groups
        // In production, use regex for proper capture group support
        if remapping.from_pattern.contains("(.*)") || remapping.from_pattern.contains("(.+)") {
            // Simplified: just handle the k8s case for now
            if module.starts_with("io.k8s.api.")
                && remapping.from_pattern.starts_with("io.k8s.api.")
            {
                let rest = module.strip_prefix("io.k8s.api.")?;
                return Some(format!("api.{}", rest));
            }
            if module.starts_with("io.k8s.apimachinery.pkg.apis.")
                && remapping
                    .from_pattern
                    .starts_with("io.k8s.apimachinery.pkg.apis.")
            {
                let rest = module.strip_prefix("io.k8s.")?;
                return Some(rest.to_string());
            }
        }
        None
    }
}

impl Default for SpecialCaseRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_transform() {
        let registry = SpecialCaseRegistry::default();
        let context = Context {
            current_module: "test".to_string(),
            source_type: "openapi".to_string(),
            metadata: HashMap::new(),
        };

        let result = registry
            .transform_type_name("io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta", &context);
        assert_eq!(result, "ObjectMeta");
    }

    #[test]
    fn test_module_remapping() {
        let registry = SpecialCaseRegistry::default();

        let result = registry.remap_module("io.k8s.api.core.v1");
        assert_eq!(result, "api.core.v1");

        let result = registry.remap_module("io.k8s.apimachinery.pkg.apis.meta.v1");
        assert_eq!(result, "apimachinery.pkg.apis.meta.v1");
    }

    #[test]
    fn test_field_rename() {
        let registry = SpecialCaseRegistry::default();

        let result = registry.get_field_rename("SomeType", "$ref");
        assert_eq!(result, Some("ref_field".to_string()));

        let result = registry.get_field_rename("AnyType", "type");
        assert_eq!(result, Some("type_field".to_string()));
    }
}
