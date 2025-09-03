//! CRD walker that produces uniform IR

use super::{DependencyGraph, SchemaWalker, TypeRegistry, WalkerError};
use amalgam_core::{
    ir::{Import, Module, TypeDefinition, IR},
    types::{Field, Type},
    ImportPathCalculator,
};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use tracing::instrument;

pub struct CRDWalker {
    /// Base module name for generated types
    base_module: String,
}

impl CRDWalker {
    pub fn new(base_module: impl Into<String>) -> Self {
        Self {
            base_module: base_module.into(),
        }
    }

    /// Convert JSON Schema from CRD to our Type representation
    #[instrument(skip(self, schema, refs), level = "trace")]
    fn json_schema_to_type(
        &self,
        schema: &Value,
        refs: &mut Vec<String>,
    ) -> Result<Type, WalkerError> {
        if let Some(ref_str) = schema.get("$ref").and_then(|v| v.as_str()) {
            // Handle reference
            refs.push(ref_str.to_string());

            // Extract type name from reference, handling #/definitions/ prefix
            let type_name = ref_str.trim_start_matches("#/definitions/");
            let type_name = type_name.rsplit('/').next().unwrap_or(type_name);

            // Check if this is a k8s reference
            let module = if ref_str.contains("io.k8s.") {
                // Extract k8s module path from the reference
                let full_name = ref_str.trim_start_matches("#/definitions/");
                if full_name.starts_with("io.k8s.") {
                    let parts: Vec<&str> = full_name.split('.').collect();
                    if parts.len() > 1 {
                        Some(parts[..parts.len() - 1].join("."))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                // Local reference within the same module
                Some(self.base_module.clone())
            };

            return Ok(Type::Reference {
                name: type_name.to_string(),
                module,
            });
        }

        let type_str = schema.get("type").and_then(|v| v.as_str());

        match type_str {
            Some("string") => Ok(Type::String),
            Some("number") => Ok(Type::Number),
            Some("integer") => Ok(Type::Integer),
            Some("boolean") => Ok(Type::Bool),
            Some("null") => Ok(Type::Null),

            Some("array") => {
                let items = schema.get("items");
                let item_type = if let Some(items_schema) = items {
                    self.json_schema_to_type(items_schema, refs)?
                } else {
                    Type::Any
                };
                Ok(Type::Array(Box::new(item_type)))
            }

            Some("object") => {
                let mut fields = BTreeMap::new();
                let required = schema
                    .get("required")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(String::from)
                            .collect::<HashSet<_>>()
                    })
                    .unwrap_or_default();

                if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
                    for (name, prop_schema) in properties {
                        let field_type = self.json_schema_to_type(prop_schema, refs)?;
                        let is_required = required.contains(name);
                        let description = prop_schema
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(String::from);

                        fields.insert(
                            name.clone(),
                            Field {
                                ty: field_type,
                                required: is_required,
                                description,
                                default: None,
                            },
                        );
                    }
                }

                let open = schema
                    .get("additionalProperties")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                Ok(Type::Record { fields, open })
            }

            None => {
                // Check for oneOf, anyOf, allOf
                if let Some(one_of) = schema.get("oneOf").and_then(|v| v.as_array()) {
                    let types: Result<Vec<_>, _> = one_of
                        .iter()
                        .map(|s| self.json_schema_to_type(s, refs))
                        .collect();

                    Ok(Type::Union {
                        types: types?,
                        coercion_hint: None,
                    })
                } else if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
                    let types: Result<Vec<_>, _> = any_of
                        .iter()
                        .map(|s| self.json_schema_to_type(s, refs))
                        .collect();

                    Ok(Type::Union {
                        types: types?,
                        coercion_hint: None,
                    })
                } else if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
                    let types: Result<Vec<_>, _> = all_of
                        .iter()
                        .map(|s| self.json_schema_to_type(s, refs))
                        .collect();

                    let types = types?;
                    if types.is_empty() {
                        return Ok(Type::Any);
                    }

                    // Try to merge the types intelligently
                    self.merge_all_of_types(types)
                } else {
                    Ok(Type::Any)
                }
            }

            _ => Ok(Type::Any),
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

/// CRD input format - simplified for now
#[derive(Debug, Clone)]
pub struct CRDInput {
    pub group: String,
    pub versions: Vec<CRDVersion>,
}

#[derive(Debug, Clone)]
pub struct CRDVersion {
    pub name: String,
    pub schema: Value,
}

impl SchemaWalker for CRDWalker {
    type Input = CRDInput;

    fn walk(&self, input: Self::Input) -> Result<IR, WalkerError> {
        // Step 1: Extract all types
        let registry = self.extract_types(&input)?;

        // Step 2: Build dependency graph
        let deps = self.build_dependencies(&registry);

        // Step 3: Generate IR with imports
        self.generate_ir(registry, deps)
    }

    fn extract_types(&self, input: &Self::Input) -> Result<TypeRegistry, WalkerError> {
        let mut registry = TypeRegistry::new();

        for version in &input.versions {
            let module_name = format!("{}.{}", input.group, version.name);

            // Extract spec schema
            if let Some(spec) = version
                .schema
                .get("openAPIV3Schema")
                .and_then(|s| s.get("properties"))
                .and_then(|p| p.get("spec"))
            {
                let mut refs = Vec::new();
                let ty = self.json_schema_to_type(spec, &mut refs)?;

                let fqn = format!("{}.Spec", module_name);
                let type_def = TypeDefinition {
                    name: "Spec".to_string(),
                    ty,
                    documentation: spec
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    annotations: Default::default(),
                };

                registry.add_type(&fqn, type_def);
            }

            // Extract status schema if present
            if let Some(status) = version
                .schema
                .get("openAPIV3Schema")
                .and_then(|s| s.get("properties"))
                .and_then(|p| p.get("status"))
            {
                let mut refs = Vec::new();
                let ty = self.json_schema_to_type(status, &mut refs)?;

                let fqn = format!("{}.Status", module_name);
                let type_def = TypeDefinition {
                    name: "Status".to_string(),
                    ty,
                    documentation: status
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    annotations: Default::default(),
                };

                registry.add_type(&fqn, type_def);
            }
        }

        Ok(registry)
    }

    fn build_dependencies(&self, registry: &TypeRegistry) -> DependencyGraph {
        let mut graph = DependencyGraph::new();

        for (fqn, type_def) in &registry.types {
            let mut refs = HashSet::new();
            self.extract_references(&type_def.ty, &mut refs);

            for ref_fqn in refs {
                // Check if this is a k8s core type reference
                if ref_fqn.starts_with("io.k8s.") {
                    // Add as external dependency
                    graph.add_dependency(fqn, &ref_fqn);
                } else if registry.types.contains_key(&ref_fqn) {
                    // Internal dependency
                    graph.add_dependency(fqn, &ref_fqn);
                }
            }
        }

        graph
    }

    fn generate_ir(
        &self,
        registry: TypeRegistry,
        deps: DependencyGraph,
    ) -> Result<IR, WalkerError> {
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
                        // Handle k8s core type imports specially
                        if dep_fqn.starts_with("io.k8s.") {
                            // Map to our k8s package structure using ImportPathCalculator
                            let import_path = self.map_k8s_import_path(&module_name, &dep_fqn);
                            let type_name = dep_fqn.rsplit('.').next().unwrap_or(&dep_fqn);

                            imports_map
                                .entry(import_path)
                                .or_default()
                                .insert(type_name.to_string());
                        } else if let Some(last_dot) = dep_fqn.rfind('.') {
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
                let import_path = if import_module.starts_with("../") {
                    // Already a path
                    import_module.clone()
                } else {
                    self.calculate_import_path(&module_name, &import_module)
                };

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

impl CRDWalker {
    /// Map k8s core type references to import paths using ImportPathCalculator
    fn map_k8s_import_path(&self, from_module: &str, fqn: &str) -> String {
        let calc = ImportPathCalculator::new();

        // Parse the current module to get from_group and from_version
        let (from_group, from_version) = Self::parse_module_name(from_module);

        // Extract type name from FQN
        let type_name = fqn.rsplit('.').next().unwrap_or("unknown").to_lowercase();

        // Map k8s FQN to (group, version)
        if fqn.starts_with("io.k8s.apimachinery.pkg.apis.meta.") {
            // Meta types are in v1: io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta -> k8s.io v1
            calc.calculate(&from_group, &from_version, "k8s.io", "v1", &type_name)
        } else if fqn.starts_with("io.k8s.api.core.") {
            // Core types: io.k8s.api.core.v1.Container -> k8s.io v1
            let parts: Vec<&str> = fqn.split('.').collect();
            let version = parts.get(4).unwrap_or(&"v1");
            calc.calculate(&from_group, &from_version, "k8s.io", version, &type_name)
        } else if fqn.starts_with("io.k8s.") {
            // Other k8s types - extract version from FQN
            let parts: Vec<&str> = fqn.split('.').collect();
            // Find version-like part (v1, v1beta1, v1alpha3, etc)
            let version = parts
                .iter()
                .find(|&&part| {
                    part.starts_with("v")
                        && (part[1..].chars().all(|c| c.is_ascii_digit())
                            || part.contains("alpha")
                            || part.contains("beta"))
                })
                .unwrap_or(&"v1");
            calc.calculate(&from_group, &from_version, "k8s.io", version, &type_name)
        } else {
            // Non-k8s types - default behavior
            calc.calculate(&from_group, &from_version, "unknown", "v1", &type_name)
        }
    }

    /// Calculate relative import path between modules
    fn calculate_import_path(&self, from_module: &str, to_module: &str) -> String {
        let calc = ImportPathCalculator::new();

        // Parse module names to extract group and version
        let (from_group, from_version) = Self::parse_module_name(from_module);
        let (to_group, to_version) = Self::parse_module_name(to_module);

        // For CRD modules, we typically import the module file (mod.ncl)
        calc.calculate(&from_group, &from_version, &to_group, &to_version, "mod")
    }

    /// Parse group and version from module name
    fn parse_module_name(module_name: &str) -> (String, String) {
        let parts: Vec<&str> = module_name.split('.').collect();

        // Try to identify version parts
        let version_pattern = |s: &str| {
            s.starts_with("v")
                && (s[1..].chars().all(|c| c.is_ascii_digit())
                    || s.contains("alpha")
                    || s.contains("beta"))
        };

        // Find version part
        if let Some(version_idx) = parts.iter().position(|&p| version_pattern(p)) {
            let version = parts[version_idx].to_string();
            let group = if version_idx > 0 {
                parts[..version_idx].join(".")
            } else {
                parts[version_idx + 1..].join(".")
            };
            return (group, version);
        }

        // Fallback
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
        if module.starts_with("../") {
            // Extract meaningful part from path
            module
                .rsplit('/')
                .find(|s| !s.is_empty() && *s != "..")
                .unwrap_or("import")
                .to_string()
        } else {
            // Use last part of module path as alias
            module.split('.').next_back().unwrap_or(module).to_string()
        }
    }
}
