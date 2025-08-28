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
}
