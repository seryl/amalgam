//! Nickel code generator with improved formatting

use crate::{Codegen, CodegenError};
use crate::resolver::{TypeResolver, ResolutionContext};
use amalgam_core::{
    types::{Field, Type},
    IR,
};
use std::fmt::Write;

pub struct NickelCodegen {
    indent_size: usize,
    resolver: TypeResolver,
}

impl NickelCodegen {
    pub fn new() -> Self {
        Self { 
            indent_size: 2,
            resolver: TypeResolver::new(),
        }
    }

    fn indent(&self, level: usize) -> String {
        " ".repeat(level * self.indent_size)
    }

    /// Format a documentation string properly
    /// Uses triple quotes for multiline, regular quotes for single line
    fn format_doc(&self, doc: &str) -> String {
        if doc.contains('\n') || doc.len() > 80 {
            // Use triple quotes for multiline or long docs
            format!("m%\"\n{}\n\"%", doc.trim())
        } else {
            // Use regular quotes for short docs
            format!("\"{}\"", doc.replace('"', "\\\""))
        }
    }

    fn type_to_nickel(&mut self, ty: &Type, module: &amalgam_core::ir::Module, indent_level: usize) -> Result<String, CodegenError> {
        match ty {
            Type::String => Ok("String".to_string()),
            Type::Number => Ok("Number".to_string()),
            Type::Integer => Ok("Number".to_string()), // Nickel uses Number for all numerics
            Type::Bool => Ok("Bool".to_string()),
            Type::Null => Ok("Null".to_string()),
            Type::Any => Ok("Dyn".to_string()),

            Type::Array(elem) => {
                let elem_type = self.type_to_nickel(elem, module, indent_level)?;
                Ok(format!("Array {}", elem_type))
            }

            Type::Map { value, .. } => {
                let value_type = self.type_to_nickel(value, module, indent_level)?;
                Ok(format!("{{ _ : {} }}", value_type))
            }

            Type::Optional(inner) => {
                let inner_type = self.type_to_nickel(inner, module, indent_level)?;
                Ok(format!("{} | Null", inner_type))
            }

            Type::Record { fields, open } => {
                if fields.is_empty() && *open {
                    return Ok("{ .. }".to_string());
                }

                let mut result = String::from("{\n");

                // Sort fields for consistent output
                let mut sorted_fields: Vec<_> = fields.iter().collect();
                sorted_fields.sort_by_key(|(name, _)| *name);

                for (name, field) in sorted_fields {
                    let field_str = self.field_to_nickel(name, field, module, indent_level + 1)?;
                    result.push_str(&field_str);
                    result.push_str(",\n");
                }

                if *open {
                    result.push_str(&format!("{}.. | Dyn,\n", self.indent(indent_level + 1)));
                }

                result.push_str(&self.indent(indent_level));
                result.push('}');
                Ok(result)
            }

            Type::Union(types) => {
                let type_strs: Result<Vec<_>, _> = types
                    .iter()
                    .map(|t| self.type_to_nickel(t, module, indent_level))
                    .collect();
                Ok(type_strs?.join(" | "))
            }

            Type::TaggedUnion {
                tag_field,
                variants,
            } => {
                let mut contracts = Vec::new();
                for (tag, variant_type) in variants {
                    let variant_str = self.type_to_nickel(variant_type, module, indent_level)?;
                    contracts.push(format!("({} == \"{}\" && {})", tag_field, tag, variant_str));
                }
                Ok(contracts.join(" | "))
            }

            Type::Reference(name) => {
                // Use the resolver to get the proper reference
                let context = ResolutionContext {
                    current_group: None,  // Could extract from module.name if needed
                    current_version: None,
                    current_kind: None,
                };
                Ok(self.resolver.resolve(name, module, &context))
            },

            Type::Contract { base, predicate } => {
                let base_type = self.type_to_nickel(base, module, indent_level)?;
                Ok(format!("{} | Contract({})", base_type, predicate))
            }
        }
    }

    fn field_to_nickel(
        &mut self,
        name: &str,
        field: &Field,
        module: &amalgam_core::ir::Module,
        indent_level: usize,
    ) -> Result<String, CodegenError> {
        let indent = self.indent(indent_level);
        let type_str = self.type_to_nickel(&field.ty, module, indent_level)?;

        let mut parts = Vec::new();

        // Field name
        parts.push(format!("{}{}", indent, name));

        // Optional modifier (put before type for readability)
        if !field.required {
            parts.push("optional".to_string());
        }

        // Type
        parts.push(type_str);

        // Default value (before doc)
        if let Some(default) = &field.default {
            let default_str = format_json_value(default, indent_level);
            parts.push(format!("default = {}", default_str));
        }

        // Documentation (always last for better readability)
        if let Some(desc) = &field.description {
            parts.push(format!("doc {}", self.format_doc(desc)));
        }

        Ok(parts.join(" | "))
    }
}

/// Format a JSON value for Nickel
fn format_json_value(value: &serde_json::Value, indent_level: usize) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr
                .iter()
                .map(|v| format_json_value(v, indent_level))
                .collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let indent = " ".repeat((indent_level + 1) * 2);
                let mut items = Vec::new();
                for (k, v) in obj {
                    items.push(format!(
                        "{}{} = {}",
                        indent,
                        k,
                        format_json_value(v, indent_level + 1)
                    ));
                }
                format!(
                    "{{\n{}\n{}}}",
                    items.join(",\n"),
                    " ".repeat(indent_level * 2)
                )
            }
        }
    }
}

impl Default for NickelCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl Codegen for NickelCodegen {
    fn generate(&mut self, ir: &IR) -> Result<String, CodegenError> {
        let mut output = String::new();

        for module in &ir.modules {
            // Module header comment
            writeln!(output, "# Module: {}", module.name)
                .map_err(|e| CodegenError::Generation(e.to_string()))?;
            writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;

            // Generate imports if any
            if !module.imports.is_empty() {
                for import in &module.imports {
                    writeln!(
                        output,
                        "let {} = import \"{}\" in",
                        import.alias.as_ref().unwrap_or(&import.path),
                        import.path
                    )
                    .map_err(|e| CodegenError::Generation(e.to_string()))?;
                }
                writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;
            }

            // Generate type definitions with proper formatting
            writeln!(output, "{{")?;

            for (idx, type_def) in module.types.iter().enumerate() {
                // Add type documentation as a comment if present
                if let Some(doc) = &type_def.documentation {
                    for line in doc.lines() {
                        writeln!(output, "{}# {}", self.indent(1), line)
                            .map_err(|e| CodegenError::Generation(e.to_string()))?;
                    }
                }

                // Generate the type with proper indentation
                let type_str = self.type_to_nickel(&type_def.ty, module, 1)?;

                // Check if type is a record that needs special formatting
                if matches!(type_def.ty, Type::Record { .. }) {
                    // For records, put the opening brace on the same line
                    write!(output, "  {} = ", type_def.name)?;
                    writeln!(output, "{},", type_str)?;
                } else {
                    writeln!(output, "  {} = {},", type_def.name, type_str)?;
                }

                // Add spacing between types for readability
                if idx < module.types.len() - 1 {
                    writeln!(output)?;
                }
            }

            // Generate constants with proper formatting
            if !module.constants.is_empty() {
                writeln!(output)?; // Add spacing before constants

                for constant in &module.constants {
                    if let Some(doc) = &constant.documentation {
                        writeln!(output, "  # {}", doc)
                            .map_err(|e| CodegenError::Generation(e.to_string()))?;
                    }

                    let value_str = format_json_value(&constant.value, 1);
                    writeln!(output, "  {} = {},", constant.name, value_str)
                        .map_err(|e| CodegenError::Generation(e.to_string()))?;
                }
            }

            writeln!(output, "}}")?;
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amalgam_core::ir::{Module, Metadata};
    use std::collections::HashMap;

    fn create_test_module() -> Module {
        Module {
            name: "test".to_string(),
            imports: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
            metadata: Metadata {
                source_language: None,
                source_file: None,
                version: None,
                generated_at: None,
                custom: HashMap::new(),
            },
        }
    }

    #[test]
    fn test_simple_type_generation() {
        let mut codegen = NickelCodegen::new();
        let module = create_test_module();

        assert_eq!(codegen.type_to_nickel(&Type::String, &module, 0).unwrap(), "String");
        assert_eq!(codegen.type_to_nickel(&Type::Number, &module, 0).unwrap(), "Number");
        assert_eq!(codegen.type_to_nickel(&Type::Bool, &module, 0).unwrap(), "Bool");
        assert_eq!(codegen.type_to_nickel(&Type::Any, &module, 0).unwrap(), "Dyn");
    }

    #[test]
    fn test_array_generation() {
        let mut codegen = NickelCodegen::new();
        let module = create_test_module();
        let array_type = Type::Array(Box::new(Type::String));
        assert_eq!(
            codegen.type_to_nickel(&array_type, &module, 0).unwrap(),
            "Array String"
        );
    }

    #[test]
    fn test_optional_generation() {
        let mut codegen = NickelCodegen::new();
        let module = create_test_module();
        let optional_type = Type::Optional(Box::new(Type::String));
        assert_eq!(
            codegen.type_to_nickel(&optional_type, &module, 0).unwrap(),
            "String | Null"
        );
    }

    #[test]
    fn test_doc_formatting() {
        let codegen = NickelCodegen::new();

        // Short doc uses regular quotes
        assert_eq!(codegen.format_doc("Short doc"), "\"Short doc\"");

        // Multiline doc uses triple quotes
        let multiline = "This is a\nmultiline doc";
        assert_eq!(
            codegen.format_doc(multiline),
            "m%\"\nThis is a\nmultiline doc\n\"%"
        );

        // Escapes quotes in short docs
        assert_eq!(
            codegen.format_doc("Doc with \"quotes\""),
            "\"Doc with \\\"quotes\\\"\""
        );
    }
}
