//! Pipeline integration for special cases
//!
//! This module provides clean integration points for special cases
//! in the main compilation pipeline, keeping the core logic clean.

use super::{Context, SpecialCaseRegistry};
use crate::ir::{Module, TypeDefinition};
use crate::types::Type;
use std::sync::Arc;

/// A pipeline stage that can apply special case transformations
#[derive(Clone)]
pub struct SpecialCasePipeline {
    registry: Arc<SpecialCaseRegistry>,
}

impl Default for SpecialCasePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl SpecialCasePipeline {
    /// Create a new pipeline with the default special cases
    pub fn new() -> Self {
        Self {
            registry: Arc::new(SpecialCaseRegistry::default()),
        }
    }

    /// Create a pipeline with a custom registry
    pub fn with_registry(registry: SpecialCaseRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Load special cases from configuration files
    pub fn from_config_dir(dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = SpecialCaseRegistry::default();

        // Load all .toml files from the directory
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let _content = std::fs::read_to_string(&path)?;
                // Merge rules from this file into the registry
                // (In production, implement proper merging logic)
            }
        }

        Ok(Self {
            registry: Arc::new(registry),
        })
    }

    /// Process a module through the special case pipeline
    pub fn process_module(&self, module: &mut Module, source_type: &str) {
        let context = Context {
            current_module: module.name.clone(),
            source_type: source_type.to_string(),
            metadata: Default::default(),
        };

        // Apply module remapping
        let remapped = self.registry.remap_module(&module.name);
        if remapped != module.name {
            tracing::debug!("Remapped module {} -> {}", module.name, remapped);
            module.name = remapped;
        }

        // Process all types in the module
        for type_def in &mut module.types {
            self.process_type_definition(type_def, &context);
        }
    }

    /// Process a type definition through special cases
    pub fn process_type_definition(&self, type_def: &mut TypeDefinition, context: &Context) {
        // Transform the type name if needed
        let transformed = self.registry.transform_type_name(&type_def.name, context);
        if transformed != type_def.name {
            tracing::debug!("Transformed type {} -> {}", type_def.name, transformed);
            type_def.name = transformed;
        }

        // Process the type recursively
        self.process_type(&mut type_def.ty, context);
    }

    /// Process a type recursively
    pub fn process_type(&self, ty: &mut Type, context: &Context) {
        match ty {
            Type::Record { fields, .. } => {
                // Process field renames
                let mut renamed_fields = vec![];
                for (field_name, _field) in fields.iter() {
                    if let Some(new_name) = self
                        .registry
                        .get_field_rename(&context.current_module, field_name)
                    {
                        tracing::debug!("Renaming field {} -> {}", field_name, new_name);
                        renamed_fields.push((field_name.clone(), new_name));
                    }
                }

                // Apply renames
                for (old_name, new_name) in renamed_fields {
                    if let Some(field) = fields.remove(&old_name) {
                        fields.insert(new_name, field);
                    }
                }

                // Recursively process field types
                for field in fields.values_mut() {
                    self.process_type(&mut field.ty, context);
                }
            }

            Type::Array(inner) => {
                self.process_type(inner, context);
            }

            Type::Optional(inner) => {
                self.process_type(inner, context);
            }

            Type::Union {
                types,
                coercion_hint,
            } => {
                // Apply coercion strategy if available
                if coercion_hint.is_none() {
                    // Check if we have a coercion strategy for this type
                    if let Some(strategy) =
                        self.registry.get_coercion_strategy(&context.current_module)
                    {
                        *coercion_hint = Some(match strategy {
                            super::CoercionStrategy::PreferString => {
                                crate::types::UnionCoercion::PreferString
                            }
                            super::CoercionStrategy::PreferNumber => {
                                crate::types::UnionCoercion::PreferNumber
                            }
                            _ => crate::types::UnionCoercion::NoPreference,
                        });
                    }
                }

                for t in types {
                    self.process_type(t, context);
                }
            }

            Type::Reference { name, module } => {
                // Transform reference names
                let transformed = self.registry.transform_type_name(name, context);
                if &transformed != name {
                    tracing::debug!("Transformed reference {} -> {}", name, transformed);
                    *name = transformed;
                }

                // Remap module if present
                if let Some(ref mut mod_name) = module {
                    let remapped = self.registry.remap_module(mod_name);
                    if remapped != *mod_name {
                        tracing::debug!("Remapped reference module {} -> {}", mod_name, remapped);
                        *mod_name = remapped;
                    }
                }
            }

            _ => {}
        }
    }

    /// Get import override for a specific type reference
    pub fn get_import_override(&self, from_module: &str, target_type: &str) -> Option<String> {
        self.registry
            .get_import_override(from_module, target_type)
            .map(|o| o.import_path.clone())
    }
}

/// Extension trait to add special case handling to existing pipelines
pub trait WithSpecialCases {
    /// Apply special case transformations
    fn apply_special_cases(&mut self, pipeline: &SpecialCasePipeline);
}

impl WithSpecialCases for Module {
    fn apply_special_cases(&mut self, pipeline: &SpecialCasePipeline) {
        pipeline.process_module(self, "unknown");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Field;
    use std::collections::BTreeMap;

    #[test]
    fn test_pipeline_field_rename() {
        let pipeline = SpecialCasePipeline::new();
        let context = Context {
            current_module: "test".to_string(),
            source_type: "openapi".to_string(),
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "$ref".to_string(),
            Field {
                ty: Type::String,
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: Vec::new(),
            },
        );

        let mut ty = Type::Record {
            fields,
            open: false,
        };

        pipeline.process_type(&mut ty, &context);

        if let Type::Record { fields, .. } = ty {
            assert!(!fields.contains_key("$ref"));
            assert!(fields.contains_key("ref_field"));
        }
    }
}
