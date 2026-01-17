//! Special Case Registry - Data-driven handling of edge cases and exceptions
//!
//! This module provides a clean, declarative way to handle special cases
//! without cluttering the main pipeline code. Special cases are defined
//! as data structures that can be composed and pipelined.

mod pipeline;

pub use pipeline::{SpecialCasePipeline, WithSpecialCases};

use crate::naming::to_camel_case as naming_to_camel_case;
use crate::types::Type;
use regex::Regex;
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

    /// Get import override if one exists (exact match)
    pub fn get_import_override(
        &self,
        from_module: &str,
        target_type: &str,
    ) -> Option<&ImportOverride> {
        self.import_overrides
            .iter()
            .find(|o| o.from_module == from_module && o.target_type == target_type)
    }

    /// Resolve import path using pattern matching
    ///
    /// This is more flexible than get_import_override as it supports wildcards
    /// in both from_module and target_type patterns.
    ///
    /// Example patterns:
    /// - from_module: "api.*" matches "api.core.v1", "api.apps.v1", etc.
    /// - target_type: "ObjectMeta" or "*" for any type
    pub fn resolve_import_path(
        &self,
        from_module: &str,
        target_type: &str,
    ) -> Option<String> {
        // First try exact match
        if let Some(override_) = self.get_import_override(from_module, target_type) {
            return Some(override_.import_path.clone());
        }

        // Then try pattern matching
        for override_ in &self.import_overrides {
            let module_matches = self.matches_pattern(&override_.from_module, from_module);
            let type_matches = self.matches_pattern(&override_.target_type, target_type);

            if module_matches && type_matches {
                // Apply any substitution patterns in the import_path
                let resolved = self.apply_path_substitutions(
                    &override_.import_path,
                    from_module,
                    target_type,
                );
                return Some(resolved);
            }
        }

        None
    }

    /// Apply substitutions to an import path template
    ///
    /// Supports placeholders like:
    /// - $version - extracts version from module (e.g., "v1" from "api.core.v1")
    /// - $type - the target type name
    fn apply_path_substitutions(
        &self,
        template: &str,
        from_module: &str,
        target_type: &str,
    ) -> String {
        let mut result = template.to_string();

        // Extract version from module (last segment that starts with 'v')
        let version = from_module
            .split('.')
            .rev()
            .find(|s| s.starts_with('v') && s.len() > 1 && s.chars().nth(1).map_or(false, |c| c.is_ascii_digit()))
            .unwrap_or("v1");

        result = result.replace("$version", version);
        result = result.replace("$type", target_type);

        result
    }

    /// Get all import overrides (for debugging/introspection)
    pub fn import_overrides(&self) -> &[ImportOverride] {
        &self.import_overrides
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
        // Handle special wildcard patterns
        if pattern == "*" {
            return true;
        }

        // Check if pattern contains regex capture groups
        if pattern.contains("(.*)") || pattern.contains("(.+)") {
            // Convert the pattern to a proper regex and check for a match
            if let Some(regex) = self.pattern_to_regex(pattern) {
                return regex.is_match(text);
            }
        }

        // Simple wildcard matching for patterns without capture groups
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

    /// Convert a pattern string with capture groups to a Regex
    fn pattern_to_regex(&self, pattern: &str) -> Option<Regex> {
        // Escape regex special characters except for our capture groups
        let mut regex_pattern = String::new();
        regex_pattern.push('^');

        let mut chars = pattern.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    // Check if this is a capture group pattern
                    let rest: String = chars.clone().take(3).collect();
                    if rest.starts_with(".*") || rest.starts_with(".+") {
                        // It's a capture group, pass through
                        regex_pattern.push('(');
                        regex_pattern.push('.');
                        chars.next(); // skip '.'
                        if let Some(quantifier) = chars.next() {
                            regex_pattern.push(quantifier);
                        }
                        if chars.peek() == Some(&')') {
                            chars.next();
                            regex_pattern.push(')');
                        }
                    } else {
                        regex_pattern.push_str("\\(");
                    }
                }
                '.' => regex_pattern.push_str("\\."),
                '*' => regex_pattern.push_str(".*"),
                '+' => regex_pattern.push_str("\\+"),
                '?' => regex_pattern.push_str("\\?"),
                '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => {
                    regex_pattern.push('\\');
                    regex_pattern.push(c);
                }
                _ => regex_pattern.push(c),
            }
        }

        regex_pattern.push('$');
        Regex::new(&regex_pattern).ok()
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
        // Check if the pattern contains capture groups
        if remapping.from_pattern.contains("(.*)") || remapping.from_pattern.contains("(.+)") {
            // Build a regex from the pattern
            if let Some(regex) = self.pattern_to_regex(&remapping.from_pattern) {
                if let Some(captures) = regex.captures(module) {
                    // Apply the to_pattern with capture group substitutions
                    let mut result = remapping.to_pattern.clone();

                    // Replace $1, $2, etc. with captured groups
                    for (i, cap) in captures.iter().enumerate().skip(1) {
                        if let Some(matched) = cap {
                            let placeholder = format!("${}", i);
                            result = result.replace(&placeholder, matched.as_str());
                        }
                    }

                    return Some(result);
                }
            }
            return None;
        }

        // For non-regex patterns, try simple prefix matching
        if let Some(prefix) = remapping.from_pattern.strip_suffix('*') {
            if module.starts_with(prefix) {
                let rest = module.strip_prefix(prefix)?;
                if let Some(to_prefix) = remapping.to_pattern.strip_suffix("$1") {
                    return Some(format!("{}{}", to_prefix, rest));
                }
            }
        }

        // Exact match
        if remapping.from_pattern == module {
            return Some(remapping.to_pattern.clone());
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

        // $ref -> ref_field (JSON Schema reference fields)
        let result = registry.get_field_rename("SomeType", "$ref");
        assert_eq!(result, Some("ref_field".to_string()));

        // "type" is no longer renamed - it should be quoted in Nickel output instead
        let result = registry.get_field_rename("AnyType", "type");
        assert_eq!(result, None);
    }

    #[test]
    fn test_resolve_import_path_with_pattern() {
        let registry = SpecialCaseRegistry::default();

        // Test ObjectMeta from api.core.v1
        let result = registry.resolve_import_path("api.core.v1", "ObjectMeta");
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(path.contains("apimachinery.pkg.apis/meta/v1/mod.ncl"));

        // Test ObjectMeta from api.apps.v1
        let result = registry.resolve_import_path("api.apps.v1", "ObjectMeta");
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(path.contains("apimachinery.pkg.apis/meta/v1/mod.ncl"));
    }

    #[test]
    fn test_resolve_import_path_runtime_types() {
        let registry = SpecialCaseRegistry::default();

        // IntOrString should resolve from any module
        let result = registry.resolve_import_path("api.core.v1", "IntOrString");
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(path.contains("runtime/v0/mod.ncl"));

        // Quantity should also resolve
        let result = registry.resolve_import_path("api.apps.v1", "Quantity");
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(path.contains("runtime/v0/mod.ncl"));
    }

    #[test]
    fn test_resolve_import_path_version_substitution() {
        let registry = SpecialCaseRegistry::default();

        // v1 module should get v1 in path
        let result = registry.resolve_import_path("api.core.v1", "LabelSelector");
        assert!(result.is_some());
        assert!(result.unwrap().contains("/v1/"));

        // v1beta1 module should get v1beta1 in path (if pattern supports it)
        // Note: currently we default to extracting version from the module
    }

    #[test]
    fn test_import_overrides_accessible() {
        let registry = SpecialCaseRegistry::default();

        // Should have multiple import overrides loaded
        assert!(!registry.import_overrides().is_empty());

        // Should have ObjectMeta override
        let has_object_meta = registry
            .import_overrides()
            .iter()
            .any(|o| o.target_type == "ObjectMeta");
        assert!(has_object_meta);
    }

    #[test]
    fn test_pattern_to_regex() {
        let registry = SpecialCaseRegistry::default();

        // Test simple capture group pattern
        let regex = registry.pattern_to_regex("io.k8s.api.(.*)");
        assert!(regex.is_some());
        let regex = regex.unwrap();
        assert!(regex.is_match("io.k8s.api.core.v1"));
        assert!(regex.is_match("io.k8s.api.apps.v1"));
        assert!(!regex.is_match("io.k8s.apimachinery.pkg.apis.meta.v1"));
    }

    #[test]
    fn test_apply_remapping_with_regex() {
        let registry = SpecialCaseRegistry::default();

        // Test that capture groups are correctly substituted
        let result = registry.remap_module("io.k8s.api.core.v1");
        assert_eq!(result, "api.core.v1");

        let result = registry.remap_module("io.k8s.api.apps.v1");
        assert_eq!(result, "api.apps.v1");

        let result = registry.remap_module("io.k8s.api.batch.v1");
        assert_eq!(result, "api.batch.v1");
    }

    #[test]
    fn test_matches_pattern_with_wildcards() {
        let registry = SpecialCaseRegistry::default();

        // Test wildcard matching
        assert!(registry.matches_pattern("*", "anything"));
        assert!(registry.matches_pattern("api.*", "api.core.v1"));
        assert!(registry.matches_pattern("*.v1", "api.core.v1"));
        assert!(registry.matches_pattern("*core*", "api.core.v1"));
    }

    #[test]
    fn test_matches_pattern_with_capture_groups() {
        let registry = SpecialCaseRegistry::default();

        // Test regex capture group matching
        assert!(registry.matches_pattern("io.k8s.api.(.*)", "io.k8s.api.core.v1"));
        assert!(registry.matches_pattern("io.k8s.api.(.*)", "io.k8s.api.apps.v1"));
        assert!(!registry.matches_pattern("io.k8s.api.(.*)", "io.k8s.apimachinery.pkg"));
    }
}
