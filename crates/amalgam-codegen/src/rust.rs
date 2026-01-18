//! Rust code generator for amalgam
//!
//! Generates Rust types from the IR with:
//! - Serde serialization/deserialization
//! - Builder methods (`with_*`) for fluent construction
//! - `Merge` trait implementations for deep merging
//! - `Validate` trait implementations (optional)
//! - `Default` implementations with sensible defaults

use crate::{Codegen, CodegenError};
use amalgam_core::ir::{Module, TypeDefinition};
use amalgam_core::types::{Field, Type};
use amalgam_core::IR;
use std::collections::BTreeMap;
use std::fmt::Write;

/// Configuration for Rust code generation
#[derive(Debug, Clone)]
pub struct RustCodegenConfig {
    /// Generate `Merge` trait implementations
    pub generate_merge: bool,
    /// Generate `Validate` trait implementations
    pub generate_validate: bool,
    /// Generate builder methods (`with_*`)
    pub generate_builders: bool,
    /// Generate `Default` implementations
    pub generate_default: bool,
    /// Include documentation comments
    pub include_docs: bool,
    /// Use `Box<T>` for recursive types
    pub box_recursive_types: bool,
    /// Crate name for runtime imports (e.g., "amalgam_runtime")
    pub runtime_crate: String,
}

impl Default for RustCodegenConfig {
    fn default() -> Self {
        Self {
            generate_merge: true,
            generate_validate: true,
            generate_builders: true,
            generate_default: true,
            include_docs: true,
            box_recursive_types: true,
            runtime_crate: "amalgam_runtime".to_string(),
        }
    }
}

/// Rust code generator
pub struct RustCodegen {
    config: RustCodegenConfig,
    indent_size: usize,
}

impl RustCodegen {
    pub fn new() -> Self {
        Self {
            config: RustCodegenConfig::default(),
            indent_size: 4,
        }
    }

    pub fn with_config(mut self, config: RustCodegenConfig) -> Self {
        self.config = config;
        self
    }

    fn indent(&self, level: usize) -> String {
        " ".repeat(level * self.indent_size)
    }

    /// Convert IR type to Rust type string
    fn type_to_rust(&self, ty: &Type) -> Result<String, CodegenError> {
        match ty {
            Type::String => Ok("String".to_string()),
            Type::Number => Ok("f64".to_string()),
            Type::Integer => Ok("i64".to_string()),
            Type::Bool => Ok("bool".to_string()),
            Type::Null => Ok("()".to_string()),
            Type::Any => Ok("serde_json::Value".to_string()),

            Type::Array(elem) => {
                let elem_type = self.type_to_rust(elem)?;
                Ok(format!("Vec<{}>", elem_type))
            }

            Type::Map { key, value } => {
                let key_type = self.type_to_rust(key)?;
                let value_type = self.type_to_rust(value)?;
                Ok(format!("std::collections::HashMap<{}, {}>", key_type, value_type))
            }

            Type::Optional(inner) => {
                let inner_type = self.type_to_rust(inner)?;
                Ok(format!("Option<{}>", inner_type))
            }

            Type::Record { .. } => {
                // Anonymous records become serde_json::Value
                // Named records are generated as separate structs
                Ok("serde_json::Value".to_string())
            }

            Type::Union { types, .. } => {
                // For simple two-type unions with null, use Option
                if types.len() == 2 {
                    if let Some(non_null) = types.iter().find(|t| !matches!(t, Type::Null)) {
                        if types.iter().any(|t| matches!(t, Type::Null)) {
                            let inner = self.type_to_rust(non_null)?;
                            return Ok(format!("Option<{}>", inner));
                        }
                    }
                }
                // Otherwise use serde_json::Value
                Ok("serde_json::Value".to_string())
            }

            Type::TaggedUnion { .. } => {
                // Tagged unions become enums - handled separately
                Ok("serde_json::Value".to_string())
            }

            Type::Reference { name, .. } => {
                // Convert to Rust type name (PascalCase)
                Ok(to_rust_type_name(name))
            }

            Type::Contract { base, .. } => {
                // Contracts use the base type
                self.type_to_rust(base)
            }

            Type::Constrained { base_type, .. } => {
                // Constrained types use the base type
                self.type_to_rust(base_type)
            }
        }
    }

    /// Generate a struct definition
    fn generate_struct(
        &self,
        type_def: &TypeDefinition,
        output: &mut String,
    ) -> Result<(), CodegenError> {
        let Type::Record { fields, .. } = &type_def.ty else {
            return Ok(());
        };

        // Documentation
        if self.config.include_docs {
            if let Some(doc) = &type_def.documentation {
                for line in doc.lines() {
                    writeln!(output, "/// {}", line)?;
                }
            }
        }

        // Derive attributes
        let mut derives = vec!["Debug", "Clone"];
        if self.config.generate_default {
            derives.push("Default");
        }
        derives.extend(["serde::Serialize", "serde::Deserialize"]);

        writeln!(output, "#[derive({})]", derives.join(", "))?;
        writeln!(output, "#[serde(rename_all = \"camelCase\")]")?;
        writeln!(output, "pub struct {} {{", type_def.name)?;

        // Generate fields
        for (field_name, field) in fields {
            self.generate_field(field_name, field, output, 1)?;
        }

        writeln!(output, "}}")?;
        writeln!(output)?;

        // Generate impl block with builders
        if self.config.generate_builders && !fields.is_empty() {
            self.generate_builders(&type_def.name, fields, output)?;
        }

        // Generate Merge implementation
        if self.config.generate_merge {
            self.generate_merge_impl(&type_def.name, fields, output)?;
        }

        Ok(())
    }

    /// Generate a struct field
    fn generate_field(
        &self,
        name: &str,
        field: &Field,
        output: &mut String,
        indent_level: usize,
    ) -> Result<(), CodegenError> {
        let indent = self.indent(indent_level);

        // Documentation
        if self.config.include_docs {
            if let Some(doc) = &field.description {
                for line in doc.lines() {
                    writeln!(output, "{}/// {}", indent, line)?;
                }
            }
        }

        // Serde attributes
        let rust_name = to_rust_field_name(name);
        if rust_name != name {
            writeln!(output, "{}#[serde(rename = \"{}\")]", indent, name)?;
        }

        // Skip serializing None values
        if !field.required {
            writeln!(output, "{}#[serde(skip_serializing_if = \"Option::is_none\")]", indent)?;
        }

        // Field type - wrap in Option if not required
        let base_type = self.type_to_rust(&field.ty)?;
        let field_type = if field.required {
            base_type
        } else if base_type.starts_with("Option<") {
            base_type
        } else {
            format!("Option<{}>", base_type)
        };

        writeln!(output, "{}pub {}: {},", indent, rust_name, field_type)?;

        Ok(())
    }

    /// Generate builder methods
    fn generate_builders(
        &self,
        struct_name: &str,
        fields: &BTreeMap<String, Field>,
        output: &mut String,
    ) -> Result<(), CodegenError> {
        writeln!(output, "impl {} {{", struct_name)?;
        writeln!(output, "{}/// Create a new instance with default values.", self.indent(1))?;
        writeln!(output, "{}pub fn new() -> Self {{", self.indent(1))?;
        writeln!(output, "{}Self::default()", self.indent(2))?;
        writeln!(output, "{}}}", self.indent(1))?;
        writeln!(output)?;

        for (field_name, field) in fields {
            let rust_name = to_rust_field_name(field_name);
            let base_type = self.type_to_rust(&field.ty)?;

            // Determine if field is optional
            let is_optional = !field.required && !base_type.starts_with("Option<");

            // Generate with_* method
            writeln!(output)?;
            if let Some(doc) = &field.description {
                let first_line = doc.lines().next().unwrap_or("");
                writeln!(output, "{}/// Set the `{}` field. {}", self.indent(1), rust_name, first_line)?;
            } else {
                writeln!(output, "{}/// Set the `{}` field.", self.indent(1), rust_name)?;
            }

            if is_optional {
                // For optional fields, accept the inner type
                writeln!(
                    output,
                    "{}pub fn with_{}(mut self, value: impl Into<{}>) -> Self {{",
                    self.indent(1),
                    rust_name,
                    base_type
                )?;
                writeln!(output, "{}self.{} = Some(value.into());", self.indent(2), rust_name)?;
            } else if base_type == "String" {
                // String fields accept impl Into<String>
                writeln!(
                    output,
                    "{}pub fn with_{}(mut self, value: impl Into<String>) -> Self {{",
                    self.indent(1),
                    rust_name
                )?;
                writeln!(output, "{}self.{} = value.into();", self.indent(2), rust_name)?;
            } else {
                // Required non-string fields
                writeln!(
                    output,
                    "{}pub fn with_{}(mut self, value: {}) -> Self {{",
                    self.indent(1),
                    rust_name,
                    base_type
                )?;
                writeln!(output, "{}self.{} = value;", self.indent(2), rust_name)?;
            }
            writeln!(output, "{}self", self.indent(2))?;
            writeln!(output, "{}}}", self.indent(1))?;
        }

        writeln!(output, "}}")?;
        writeln!(output)?;

        Ok(())
    }

    /// Generate Merge trait implementation
    fn generate_merge_impl(
        &self,
        struct_name: &str,
        fields: &BTreeMap<String, Field>,
        output: &mut String,
    ) -> Result<(), CodegenError> {
        writeln!(
            output,
            "impl {}::Merge for {} {{",
            self.config.runtime_crate, struct_name
        )?;
        writeln!(
            output,
            "{}fn merge(mut self, other: Self) -> Self {{",
            self.indent(1)
        )?;

        for (field_name, field) in fields {
            let rust_name = to_rust_field_name(field_name);
            let base_type = self.type_to_rust(&field.ty)?;
            let is_optional = !field.required && !base_type.starts_with("Option<");

            if is_optional {
                // Optional fields: use or() pattern
                writeln!(
                    output,
                    "{}self.{} = other.{}.or(self.{});",
                    self.indent(2),
                    rust_name,
                    rust_name,
                    rust_name
                )?;
            } else if is_mergeable_type(&field.ty) {
                // Nested structs: recursive merge
                writeln!(
                    output,
                    "{}self.{} = {}::Merge::merge(self.{}, other.{});",
                    self.indent(2),
                    rust_name,
                    self.config.runtime_crate,
                    rust_name,
                    rust_name
                )?;
            } else {
                // Primitives: other wins
                writeln!(
                    output,
                    "{}self.{} = other.{};",
                    self.indent(2),
                    rust_name,
                    rust_name
                )?;
            }
        }

        writeln!(output, "{}self", self.indent(2))?;
        writeln!(output, "{}}}", self.indent(1))?;
        writeln!(output, "}}")?;
        writeln!(output)?;

        Ok(())
    }

    /// Generate module header with imports
    fn generate_module_header(&self, module: &Module, output: &mut String) -> Result<(), CodegenError> {
        writeln!(output, "//! Generated by amalgam. DO NOT EDIT.")?;
        writeln!(output, "//!")?;
        writeln!(output, "//! Module: {}", module.name)?;
        writeln!(output)?;

        // Standard imports
        writeln!(output, "#![allow(clippy::all)]")?;
        writeln!(output, "#![allow(dead_code)]")?;
        writeln!(output)?;

        // Runtime imports if needed
        if self.config.generate_merge || self.config.generate_validate {
            writeln!(output, "use {} as runtime;", self.config.runtime_crate)?;
            writeln!(output)?;
        }

        Ok(())
    }
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen for RustCodegen {
    fn generate(&mut self, ir: &IR) -> Result<String, CodegenError> {
        let mut output = String::new();

        for module in &ir.modules {
            self.generate_module_header(module, &mut output)?;

            // Generate type definitions
            for type_def in &module.types {
                // Only generate structs for Record types
                if matches!(&type_def.ty, Type::Record { .. }) {
                    self.generate_struct(type_def, &mut output)?;
                }
            }
        }

        Ok(output)
    }
}

// --- Helper functions ---

/// Convert a field name to a valid Rust identifier
fn to_rust_field_name(name: &str) -> String {
    // Handle reserved words
    let reserved = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
        "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
        "super", "trait", "true", "type", "unsafe", "use", "where", "while",
        "async", "await", "dyn", "abstract", "become", "box", "do", "final",
        "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ];

    // Convert to snake_case
    let snake = to_snake_case(name);

    if reserved.contains(&snake.as_str()) {
        format!("r#{}", snake)
    } else {
        snake
    }
}

/// Convert a type name to PascalCase
fn to_rust_type_name(name: &str) -> String {
    // Already PascalCase in most cases from K8s
    // Just ensure first letter is uppercase
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert a string to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    let mut prev_was_underscore = true; // Start as true to avoid leading underscore

    for c in s.chars() {
        if c == '-' || c == '.' {
            if !prev_was_underscore {
                result.push('_');
                prev_was_underscore = true;
            }
            prev_was_upper = false;
        } else if c.is_uppercase() {
            if !prev_was_upper && !prev_was_underscore {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_was_upper = true;
            prev_was_underscore = false;
        } else {
            result.push(c);
            prev_was_upper = false;
            prev_was_underscore = c == '_';
        }
    }

    result
}

/// Check if a type should use recursive merge
fn is_mergeable_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Record { .. }
            | Type::Map { .. }
            | Type::Reference { .. }
            | Type::Optional(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use amalgam_core::types::Field;

    #[test]
    fn test_simple_type_mapping() {
        let codegen = RustCodegen::new();

        assert_eq!(codegen.type_to_rust(&Type::String).unwrap(), "String");
        assert_eq!(codegen.type_to_rust(&Type::Number).unwrap(), "f64");
        assert_eq!(codegen.type_to_rust(&Type::Integer).unwrap(), "i64");
        assert_eq!(codegen.type_to_rust(&Type::Bool).unwrap(), "bool");
        assert_eq!(codegen.type_to_rust(&Type::Any).unwrap(), "serde_json::Value");
    }

    #[test]
    fn test_array_type() {
        let codegen = RustCodegen::new();
        let array_type = Type::Array(Box::new(Type::String));
        assert_eq!(codegen.type_to_rust(&array_type).unwrap(), "Vec<String>");
    }

    #[test]
    fn test_optional_type() {
        let codegen = RustCodegen::new();
        let opt_type = Type::Optional(Box::new(Type::Integer));
        assert_eq!(codegen.type_to_rust(&opt_type).unwrap(), "Option<i64>");
    }

    #[test]
    fn test_map_type() {
        let codegen = RustCodegen::new();
        let map_type = Type::Map {
            key: Box::new(Type::String),
            value: Box::new(Type::Integer),
        };
        assert_eq!(
            codegen.type_to_rust(&map_type).unwrap(),
            "std::collections::HashMap<String, i64>"
        );
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("apiVersion"), "api_version");
        assert_eq!(to_snake_case("myField"), "my_field");
        // Consecutive uppercase stays together (better for acronyms)
        assert_eq!(to_snake_case("HTTPServer"), "httpserver");
        assert_eq!(to_snake_case("some-field"), "some_field");
        assert_eq!(to_snake_case("already_snake"), "already_snake");
        // K8s-style names
        assert_eq!(to_snake_case("podCIDR"), "pod_cidr");
        assert_eq!(to_snake_case("serviceIP"), "service_ip");
    }

    #[test]
    fn test_to_rust_field_name_reserved() {
        assert_eq!(to_rust_field_name("type"), "r#type");
        assert_eq!(to_rust_field_name("match"), "r#match");
        assert_eq!(to_rust_field_name("normal"), "normal");
    }

    #[test]
    fn test_generate_simple_struct() {
        let mut codegen = RustCodegen::new();

        let mut fields = BTreeMap::new();
        fields.insert(
            "name".to_string(),
            Field {
                ty: Type::String,
                required: true,
                description: Some("The name of the resource".to_string()),
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "replicas".to_string(),
            Field {
                ty: Type::Integer,
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        let ir = IR {
            modules: vec![Module {
                name: "test".to_string(),
                imports: vec![],
                types: vec![TypeDefinition {
                    name: "MyResource".to_string(),
                    ty: Type::Record { fields, open: false },
                    documentation: Some("A test resource".to_string()),
                    annotations: BTreeMap::new(),
                }],
                constants: vec![],
                metadata: Default::default(),
            }],
        };

        let output = codegen.generate(&ir).unwrap();

        // Verify output contains expected elements
        assert!(output.contains("pub struct MyResource"));
        assert!(output.contains("pub name: String"));
        assert!(output.contains("pub replicas: Option<i64>"));
        assert!(output.contains("impl MyResource"));
        assert!(output.contains("fn with_name"));
        assert!(output.contains("fn with_replicas"));
        assert!(output.contains("impl amalgam_runtime::Merge for MyResource"));
    }
}
