//! OpenAPI schema walker that produces uniform IR

use super::{DependencyGraph, SchemaWalker, TypeRegistry, WalkerError};
use amalgam_core::{
    ir::{Import, Module, TypeDefinition, IR},
    types::{Field, Type},
    ImportPathCalculator,
};
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type as OpenAPIType};
use std::collections::{BTreeMap, HashMap, HashSet};
use tracing::{debug, instrument, trace};

pub struct OpenAPIWalker {
    /// Base module name for generated types
    base_module: String,
}

impl OpenAPIWalker {
    pub fn new(base_module: impl Into<String>) -> Self {
        Self {
            base_module: base_module.into(),
        }
    }

    /// Convert OpenAPI schema to our Type representation
    #[instrument(skip(self, refs), level = "trace")]
    fn schema_to_type(&self, schema: &Schema, refs: &mut Vec<String>) -> Result<Type, WalkerError> {
        match &schema.schema_kind {
            SchemaKind::Type(OpenAPIType::String(_)) => Ok(Type::String),
            SchemaKind::Type(OpenAPIType::Number(_)) => Ok(Type::Number),
            SchemaKind::Type(OpenAPIType::Integer(_)) => Ok(Type::Integer),
            SchemaKind::Type(OpenAPIType::Boolean(_)) => Ok(Type::Bool),

            SchemaKind::Type(OpenAPIType::Array(array_type)) => {
                let item_type = if let Some(items) = &array_type.items {
                    match items {
                        ReferenceOr::Item(schema) => self.schema_to_type(schema, refs)?,
                        ReferenceOr::Reference { reference } => {
                            refs.push(reference.clone());
                            let type_name = reference.rsplit('/').next().unwrap_or(reference);
                            Type::Reference {
                                name: type_name.to_string(),
                                module: None, // Same module reference
                            }
                        }
                    }
                } else {
                    Type::Any
                };
                Ok(Type::Array(Box::new(item_type)))
            }

            SchemaKind::Type(OpenAPIType::Object(obj)) => {
                let mut fields = BTreeMap::new();

                for (name, prop) in &obj.properties {
                    if let ReferenceOr::Item(schema) = prop {
                        let field_type = self.schema_to_type(schema, refs)?;
                        let required = obj.required.contains(name);

                        fields.insert(
                            name.clone(),
                            Field {
                                ty: field_type,
                                required,
                                description: schema.schema_data.description.clone(),
                                default: None,
                            },
                        );
                    } else if let ReferenceOr::Reference { reference } = prop {
                        // Track reference for dependency resolution
                        refs.push(reference.clone());

                        // Extract type name from reference like "#/components/schemas/TypeName"
                        let type_name = reference.rsplit('/').next().unwrap_or(reference);

                        // For OpenAPI references within the same schema (like #/components/schemas/X),
                        // they're all in the same module, so module should be None
                        fields.insert(
                            name.clone(),
                            Field {
                                ty: Type::Reference {
                                    name: type_name.to_string(),
                                    module: None, // Same module reference
                                },
                                required: obj.required.contains(name),
                                description: None,
                                default: None,
                            },
                        );
                    }
                }

                Ok(Type::Record {
                    fields,
                    open: obj.additional_properties.is_some(),
                })
            }

            SchemaKind::OneOf { one_of } => {
                let mut types = Vec::new();

                for schema_ref in one_of {
                    match schema_ref {
                        ReferenceOr::Item(schema) => {
                            types.push(self.schema_to_type(schema, refs)?);
                        }
                        ReferenceOr::Reference { reference } => {
                            refs.push(reference.clone());
                            let type_name = reference.rsplit('/').next().unwrap_or(reference);
                            types.push(Type::Reference {
                                name: type_name.to_string(),
                                module: None, // Same module reference
                            });
                        }
                    }
                }

                Ok(Type::Union {
                    types,
                    coercion_hint: None,
                })
            }

            SchemaKind::AllOf { all_of } => {
                // allOf represents intersection - all schemas must be valid
                // In our type system, we'll merge object types and create unions for conflicting types
                let mut types = Vec::new();

                for schema_ref in all_of {
                    match schema_ref {
                        ReferenceOr::Item(schema) => {
                            types.push(self.schema_to_type(schema, refs)?);
                        }
                        ReferenceOr::Reference { reference } => {
                            refs.push(reference.clone());
                            let type_name = reference.rsplit('/').next().unwrap_or(reference);
                            types.push(Type::Reference {
                                name: type_name.to_string(),
                                module: None, // Same module reference
                            });
                        }
                    }
                }

                if types.is_empty() {
                    return Ok(Type::Any);
                }

                // Try to merge the types intelligently
                self.merge_all_of_types(types)
            }

            SchemaKind::AnyOf { any_of } => {
                // anyOf represents union - at least one schema must be valid
                // This is similar to oneOf but more permissive
                let mut types = Vec::new();

                for schema_ref in any_of {
                    match schema_ref {
                        ReferenceOr::Item(schema) => {
                            types.push(self.schema_to_type(schema, refs)?);
                        }
                        ReferenceOr::Reference { reference } => {
                            refs.push(reference.clone());
                            let type_name = reference.rsplit('/').next().unwrap_or(reference);
                            types.push(Type::Reference {
                                name: type_name.to_string(),
                                module: None, // Same module reference
                            });
                        }
                    }
                }

                Ok(Type::Union {
                    types,
                    coercion_hint: None,
                })
            }

            SchemaKind::Not { .. } => {
                // Not supported in our type system
                Ok(Type::Any)
            }

            SchemaKind::Any(_) => Ok(Type::Any),
        }
    }

    /// Merge allOf types intelligently
    #[instrument(skip(self, types), level = "trace")]
    fn merge_all_of_types(&self, types: Vec<Type>) -> Result<Type, WalkerError> {
        use std::collections::BTreeMap;

        if types.len() == 1 {
            return Ok(types.into_iter().next().unwrap());
        }

        // Separate record types from other types
        let mut record_types = Vec::new();
        let mut other_types = Vec::new();

        for ty in types {
            match ty {
                Type::Record { .. } => record_types.push(ty),
                _ => other_types.push(ty),
            }
        }

        // If we have record types, merge their fields
        let merged_record = if !record_types.is_empty() {
            let mut merged_fields: BTreeMap<String, amalgam_core::types::Field> = BTreeMap::new();
            let mut is_open = false;

            for record in record_types {
                if let Type::Record { fields, open } = record {
                    is_open = is_open || open;
                    for (field_name, field) in fields {
                        // If field already exists, we need to handle conflicts
                        if let Some(existing_field) = merged_fields.get(&field_name) {
                            // For now, if there's a conflict, make it a union
                            if existing_field.ty != field.ty {
                                merged_fields.insert(
                                    field_name,
                                    amalgam_core::types::Field {
                                        ty: Type::Union {
                                            types: vec![existing_field.ty.clone(), field.ty],
                                            coercion_hint: None,
                                        },
                                        required: existing_field.required && field.required,
                                        default: field
                                            .default
                                            .or_else(|| existing_field.default.clone()),
                                        description: field
                                            .description
                                            .or_else(|| existing_field.description.clone()),
                                    },
                                );
                            }
                        } else {
                            merged_fields.insert(field_name, field);
                        }
                    }
                }
            }

            Some(Type::Record {
                fields: merged_fields,
                open: is_open,
            })
        } else {
            None
        };

        // Combine the merged record with other types
        let mut final_types = Vec::new();
        if let Some(record) = merged_record {
            final_types.push(record);
        }
        final_types.extend(other_types);

        // If we have only one type, return it directly
        if final_types.len() == 1 {
            Ok(final_types.into_iter().next().unwrap())
        } else {
            // Multiple types that can't be merged - create a union
            Ok(Type::Union {
                types: final_types,
                coercion_hint: None,
            })
        }
    }

    /// Extract references from a type recursively
    #[allow(clippy::only_used_in_recursion)]
    #[instrument(skip(self, refs), level = "trace")]
    fn extract_references(&self, ty: &Type, refs: &mut HashSet<String>) {
        match ty {
            Type::Reference { name, module } => {
                let fqn = if let Some(m) = module {
                    format!("{}.{}", m, name)
                } else {
                    name.clone()
                };
                refs.insert(fqn);
            }
            Type::Array(inner) => self.extract_references(inner, refs),
            Type::Optional(inner) => self.extract_references(inner, refs),
            Type::Map { value, .. } => self.extract_references(value, refs),
            Type::Record { fields, .. } => {
                for field in fields.values() {
                    self.extract_references(&field.ty, refs);
                }
            }
            Type::Union { types, .. } => {
                for t in types {
                    self.extract_references(t, refs);
                }
            }
            Type::TaggedUnion { variants, .. } => {
                for t in variants.values() {
                    self.extract_references(t, refs);
                }
            }
            Type::Contract { base, .. } => self.extract_references(base, refs),
            _ => {}
        }
    }
}

impl SchemaWalker for OpenAPIWalker {
    type Input = OpenAPI;

    #[instrument(skip(self, input), level = "debug")]
    fn walk(&self, input: Self::Input) -> Result<IR, WalkerError> {
        debug!("Walking OpenAPI schema");
        // Step 1: Extract all types
        let registry = self.extract_types(&input)?;
        trace!("Extracted {} types", registry.types.len());

        // Step 2: Build dependency graph
        let deps = self.build_dependencies(&registry);

        // Step 3: Generate IR with imports
        self.generate_ir(registry, deps)
    }

    #[instrument(skip(self, input), level = "debug")]
    fn extract_types(&self, input: &Self::Input) -> Result<TypeRegistry, WalkerError> {
        debug!("Extracting types from OpenAPI schema");
        let mut registry = TypeRegistry::new();

        if let Some(components) = &input.components {
            for (name, schema_ref) in &components.schemas {
                if let ReferenceOr::Item(schema) = schema_ref {
                    let mut refs = Vec::new();
                    let ty = self.schema_to_type(schema, &mut refs)?;

                    let fqn = format!("{}.{}", self.base_module, name);
                    let type_def = TypeDefinition {
                        name: name.clone(),
                        ty,
                        documentation: schema.schema_data.description.clone(),
                        annotations: Default::default(),
                    };

                    registry.add_type(&fqn, type_def);
                }
            }
        }

        Ok(registry)
    }

    #[instrument(skip(self, registry), level = "debug")]
    fn build_dependencies(&self, registry: &TypeRegistry) -> DependencyGraph {
        debug!("Building dependency graph");
        let mut graph = DependencyGraph::new();

        for (fqn, type_def) in &registry.types {
            let mut refs = HashSet::new();
            self.extract_references(&type_def.ty, &mut refs);

            for ref_fqn in refs {
                // Only add if the referenced type exists in our registry
                if registry.types.contains_key(&ref_fqn) {
                    graph.add_dependency(fqn, &ref_fqn);
                }
            }
        }

        graph
    }

    #[instrument(skip(self, registry, deps), level = "debug")]
    fn generate_ir(
        &self,
        registry: TypeRegistry,
        deps: DependencyGraph,
    ) -> Result<IR, WalkerError> {
        debug!("Generating IR from registry and dependencies");
        let mut ir = IR::new();

        // Group types by module
        for (module_name, type_names) in registry.modules {
            let mut module = Module {
                name: module_name.clone(),
                imports: Vec::new(),
                types: Vec::new(),
                constants: Vec::new(),
                metadata: Default::default(),
            };

            // Collect all imports needed for this module
            let mut imports_map: HashMap<String, HashSet<String>> = HashMap::new();

            for type_name in &type_names {
                let fqn = format!("{}.{}", module_name, type_name);

                if let Some(type_def) = registry.types.get(&fqn) {
                    module.types.push(type_def.clone());

                    // Get cross-module dependencies
                    for dep_fqn in deps.get_cross_module_deps(&fqn) {
                        // Extract module and type from dependency FQN
                        if let Some(last_dot) = dep_fqn.rfind('.') {
                            let dep_module = &dep_fqn[..last_dot];
                            let dep_type = &dep_fqn[last_dot + 1..];

                            imports_map
                                .entry(dep_module.to_string())
                                .or_default()
                                .insert(dep_type.to_string());
                        }
                    }
                }
            }

            // Convert imports map to Import structs
            for (import_module, import_types) in imports_map {
                let import_path = self.calculate_import_path(&module_name, &import_module);

                module.imports.push(Import {
                    path: import_path,
                    alias: Some(self.generate_alias(&import_module)),
                    items: import_types.into_iter().collect(),
                });
            }

            ir.add_module(module);
        }

        Ok(ir)
    }
}

impl OpenAPIWalker {
    /// Calculate relative import path between modules
    fn calculate_import_path(&self, from_module: &str, to_module: &str) -> String {
        let calc = ImportPathCalculator::new_standalone();

        // Parse module names to extract group and version
        let (from_group, from_version) = Self::parse_module_name(from_module);
        let (to_group, to_version) = Self::parse_module_name(to_module);

        // For OpenAPI, we typically import the module file (mod.ncl)
        // So we use "mod" as the type name
        calc.calculate(&from_group, &from_version, &to_group, &to_version, "mod")
    }

    /// Parse group and version from module name
    fn parse_module_name(module_name: &str) -> (String, String) {
        let parts: Vec<&str> = module_name.split('.').collect();

        // Try to identify version parts (v1, v1beta1, v1alpha1, v2, etc.)
        let version_pattern = |s: &str| {
            s.starts_with("v")
                && (s[1..].chars().all(|c| c.is_ascii_digit())
                    || s.contains("alpha")
                    || s.contains("beta"))
        };

        // Find the version part
        if let Some(version_idx) = parts.iter().position(|&p| version_pattern(p)) {
            let version = parts[version_idx].to_string();
            let group = if version_idx > 0 {
                parts[..version_idx].join(".")
            } else {
                parts[version_idx + 1..].join(".")
            };
            return (group, version);
        }

        // Fallback: treat last part as version
        if parts.len() >= 2 {
            let version = parts[parts.len() - 1].to_string();
            let group = parts[..parts.len() - 1].join(".");
            (group, version)
        } else {
            (module_name.to_string(), String::new())
        }
    }

    /// Generate an alias for an imported module
    fn generate_alias(&self, module: &str) -> String {
        // Use last part of module path as alias
        module.split('.').next_back().unwrap_or(module).to_string()
    }
}
