//! Intermediate representation for cross-language transformations

use crate::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Intermediate representation of a schema/module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IR {
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub imports: Vec<Import>,
    pub types: Vec<TypeDefinition>,
    pub constants: Vec<Constant>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub alias: Option<String>,
    pub items: Vec<String>, // Specific items to import
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub name: String,
    pub ty: Type,
    pub documentation: Option<String>,
    pub annotations: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub ty: Type,
    pub value: serde_json::Value,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub source_language: Option<String>,
    pub source_file: Option<String>,
    pub version: Option<String>,
    pub generated_at: Option<String>,
    pub custom: BTreeMap<String, serde_json::Value>,
}

impl IR {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn find_type(&self, name: &str) -> Option<&TypeDefinition> {
        self.modules
            .iter()
            .flat_map(|m| &m.types)
            .find(|t| t.name == name)
    }

    pub fn merge(mut self, other: IR) -> Self {
        self.modules.extend(other.modules);
        self
    }

    /// Deduplicate types across all modules
    pub fn deduplicate_types(&mut self) {
        for module in &mut self.modules {
            module.deduplicate_types();
        }
    }
}

impl Module {
    /// Deduplicate types within this module
    ///
    /// When duplicate type names are found:
    /// 1. If one is a list type (has `items` field that is an Array), rename it to `{Name}List`
    /// 2. Otherwise, keep only the first definition
    pub fn deduplicate_types(&mut self) {
        use std::collections::HashMap;

        // First pass: identify duplicates and categorize them
        let mut seen: HashMap<String, Vec<(usize, bool)>> = HashMap::new();

        for (idx, type_def) in self.types.iter().enumerate() {
            let is_list_type = is_k8s_list_type(&type_def.ty);
            seen.entry(type_def.name.clone())
                .or_default()
                .push((idx, is_list_type));
        }

        // Second pass: rename list types that have duplicates
        let mut renames: Vec<(usize, String)> = Vec::new();
        let mut removes: Vec<usize> = Vec::new();

        for (name, occurrences) in &seen {
            if occurrences.len() > 1 {
                // We have duplicates
                let mut kept_non_list = false;

                for (idx, is_list) in occurrences {
                    if *is_list {
                        // This is a list type - rename it
                        let new_name = format!("{}List", name);
                        renames.push((*idx, new_name));
                    } else if !kept_non_list {
                        // Keep the first non-list type
                        kept_non_list = true;
                    } else {
                        // Duplicate non-list type - remove it
                        removes.push(*idx);
                    }
                }
            }
        }

        // Apply renames
        for (idx, new_name) in renames {
            self.types[idx].name = new_name;
        }

        // Remove duplicates (in reverse order to preserve indices)
        removes.sort_by(|a, b| b.cmp(a));
        for idx in removes {
            self.types.remove(idx);
        }
    }
}

/// Check if a type is a K8s list type (has items array and metadata fields)
fn is_k8s_list_type(ty: &Type) -> bool {
    if let Type::Record { fields, .. } = ty {
        // K8s list types have: apiVersion, kind, items (Array), metadata
        let has_items_array = fields.get("items").map_or(false, |f| {
            matches!(f.ty, Type::Array(_))
        });
        let has_api_version = fields.contains_key("apiVersion");
        let has_kind = fields.contains_key("kind");
        let has_metadata = fields.contains_key("metadata");

        // Must have items array and at least apiVersion/kind to be a list type
        has_items_array && has_api_version && has_kind && has_metadata
    } else {
        false
    }
}

impl Default for IR {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder pattern for constructing IR
pub struct IRBuilder {
    ir: IR,
    current_module: Option<Module>,
}

impl IRBuilder {
    pub fn new() -> Self {
        Self {
            ir: IR::new(),
            current_module: None,
        }
    }

    pub fn module(mut self, name: impl Into<String>) -> Self {
        if let Some(module) = self.current_module.take() {
            self.ir.add_module(module);
        }
        self.current_module = Some(Module {
            name: name.into(),
            imports: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
            metadata: Metadata::default(),
        });
        self
    }

    pub fn add_type(mut self, name: impl Into<String>, ty: Type) -> Self {
        if let Some(ref mut module) = self.current_module {
            module.types.push(TypeDefinition {
                name: name.into(),
                ty,
                documentation: None,
                annotations: BTreeMap::new(),
            });
        }
        self
    }

    pub fn add_import(mut self, path: impl Into<String>) -> Self {
        if let Some(ref mut module) = self.current_module {
            module.imports.push(Import {
                path: path.into(),
                alias: None,
                items: Vec::new(),
            });
        }
        self
    }

    pub fn build(mut self) -> IR {
        if let Some(module) = self.current_module.take() {
            self.ir.add_module(module);
        }
        self.ir
    }
}

impl Default for IRBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Field;

    #[test]
    fn test_ir_builder() {
        let ir = IRBuilder::new()
            .module("test")
            .add_type("MyType", Type::String)
            .add_type("MyNumber", Type::Number)
            .build();

        assert_eq!(ir.modules.len(), 1);
        assert_eq!(ir.modules[0].name, "test");
        assert_eq!(ir.modules[0].types.len(), 2);

        let my_type = ir.find_type("MyType");
        assert!(my_type.is_some());
        assert_eq!(my_type.unwrap().ty, Type::String);
    }

    fn make_k8s_resource_type() -> Type {
        // Simulates a K8s resource like Deployment
        let mut fields = BTreeMap::new();
        fields.insert(
            "apiVersion".to_string(),
            Field {
                ty: Type::String,
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "kind".to_string(),
            Field {
                ty: Type::String,
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "metadata".to_string(),
            Field {
                ty: Type::Reference {
                    name: "ObjectMeta".to_string(),
                    module: None,
                },
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "spec".to_string(),
            Field {
                ty: Type::Record {
                    fields: BTreeMap::new(),
                    open: true,
                },
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        Type::Record { fields, open: false }
    }

    fn make_k8s_list_type() -> Type {
        // Simulates a K8s list type like DeploymentList
        let mut fields = BTreeMap::new();
        fields.insert(
            "apiVersion".to_string(),
            Field {
                ty: Type::String,
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "kind".to_string(),
            Field {
                ty: Type::String,
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "metadata".to_string(),
            Field {
                ty: Type::Reference {
                    name: "ListMeta".to_string(),
                    module: None,
                },
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "items".to_string(),
            Field {
                ty: Type::Array(Box::new(Type::Reference {
                    name: "Deployment".to_string(),
                    module: None,
                })),
                required: true,
                description: Some("Items is the list of Deployments.".to_string()),
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        Type::Record { fields, open: false }
    }

    #[test]
    fn test_is_k8s_list_type() {
        let resource = make_k8s_resource_type();
        let list = make_k8s_list_type();

        assert!(!is_k8s_list_type(&resource));
        assert!(is_k8s_list_type(&list));
    }

    #[test]
    fn test_module_deduplicate_types_renames_list() {
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![
                TypeDefinition {
                    name: "Deployment".to_string(),
                    ty: make_k8s_resource_type(),
                    documentation: Some("A Deployment resource".to_string()),
                    annotations: BTreeMap::new(),
                },
                TypeDefinition {
                    name: "Deployment".to_string(), // Duplicate name!
                    ty: make_k8s_list_type(),
                    documentation: Some("A list of Deployments".to_string()),
                    annotations: BTreeMap::new(),
                },
            ],
            constants: vec![],
            metadata: Metadata::default(),
        };

        module.deduplicate_types();

        // Should have 2 types now
        assert_eq!(module.types.len(), 2);

        // One should be Deployment, one should be DeploymentList
        let names: Vec<_> = module.types.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"Deployment"));
        assert!(names.contains(&"DeploymentList"));
    }

    #[test]
    fn test_module_deduplicate_removes_true_duplicates() {
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![
                TypeDefinition {
                    name: "MyType".to_string(),
                    ty: Type::String,
                    documentation: Some("First".to_string()),
                    annotations: BTreeMap::new(),
                },
                TypeDefinition {
                    name: "MyType".to_string(), // Duplicate with same structure
                    ty: Type::String,
                    documentation: Some("Second".to_string()),
                    annotations: BTreeMap::new(),
                },
            ],
            constants: vec![],
            metadata: Metadata::default(),
        };

        module.deduplicate_types();

        // Should have only 1 type now (first one kept)
        assert_eq!(module.types.len(), 1);
        assert_eq!(module.types[0].name, "MyType");
        assert_eq!(
            module.types[0].documentation,
            Some("First".to_string())
        );
    }

    #[test]
    fn test_ir_deduplicate_types() {
        let mut ir = IRBuilder::new()
            .module("test.v1")
            .add_type("Unique", Type::String)
            .build();

        // Manually add duplicate types
        ir.modules[0].types.push(TypeDefinition {
            name: "Deployment".to_string(),
            ty: make_k8s_resource_type(),
            documentation: None,
            annotations: BTreeMap::new(),
        });
        ir.modules[0].types.push(TypeDefinition {
            name: "Deployment".to_string(),
            ty: make_k8s_list_type(),
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.deduplicate_types();

        // Should have 3 types: Unique, Deployment, DeploymentList
        assert_eq!(ir.modules[0].types.len(), 3);

        let names: Vec<_> = ir.modules[0].types.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"Unique"));
        assert!(names.contains(&"Deployment"));
        assert!(names.contains(&"DeploymentList"));
    }
}
