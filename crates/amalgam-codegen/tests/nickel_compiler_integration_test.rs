//! Integration tests that validate generated Nickel code with the actual Nickel compiler
//!
//! These tests ensure that the Nickel code we generate is syntactically valid and
//! can be type-checked by the Nickel compiler.

use amalgam_codegen::nickel::NickelCodegen;
use amalgam_codegen::Codegen;
use amalgam_core::ir::IRBuilder;
use amalgam_core::module_registry::ModuleRegistry;
use amalgam_core::types::{Field, Type};
use std::collections::BTreeMap;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;
use tempfile::NamedTempFile;

/// Check if the nickel command is available
fn nickel_available() -> bool {
    Command::new("nickel")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run `nickel typecheck` on the given content and return the result
///
/// The content is wrapped in a record if needed to make it valid standalone Nickel
fn nickel_check(content: &str) -> Result<(), String> {
    // The codegen output may contain:
    // 1. Standalone assignments like "TypeName = { ... }" - need wrapping
    // 2. Already wrapped records like "{ TypeA = ..., TypeB = ... }" - don't wrap
    // 3. Module output with comments - need to handle module header

    let trimmed = content.trim();

    // Remove module comment header if present
    let content_without_header: String = trimmed
        .lines()
        .skip_while(|line| line.starts_with('#') || line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let content_to_check = content_without_header.trim();

    // Check if it's already a valid record expression
    let final_content = if content_to_check.starts_with('{') && content_to_check.ends_with('}') {
        // Already a record, use as-is
        content_to_check.to_string()
    } else {
        // Wrap in a record
        format!("{{\n{}\n}}", content_to_check)
    };

    let mut file = NamedTempFile::new().map_err(|e| format!("Failed to create temp file: {}", e))?;
    file.write_all(final_content.as_bytes())
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    let output = Command::new("nickel")
        .arg("typecheck")
        .arg(file.path())
        .output()
        .map_err(|e| format!("Failed to run nickel: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Nickel typecheck failed:\n{}\n\nFinal content:\n{}", stderr, final_content))
    }
}

/// Helper to create a simple record type
fn make_record_type(fields: Vec<(&str, Type, bool)>) -> Type {
    let mut field_map = BTreeMap::new();
    for (name, ty, required) in fields {
        field_map.insert(
            name.to_string(),
            Field {
                ty,
                required,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
    }
    Type::Record {
        fields: field_map,
        open: false,
    }
}

#[test]
fn test_generated_simple_record_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type(
            "SimpleRecord",
            make_record_type(vec![
                ("name", Type::String, true),
                ("count", Type::Integer, false),
                ("enabled", Type::Bool, false),
            ]),
        )
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    // The output should be valid Nickel
    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}

#[test]
fn test_generated_nested_record_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    let inner_type = make_record_type(vec![
        ("host", Type::String, true),
        ("port", Type::Integer, true),
    ]);

    let outer_type = make_record_type(vec![
        ("name", Type::String, true),
        ("server", inner_type, true),
    ]);

    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type("Config", outer_type)
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}

#[test]
fn test_generated_array_type_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type(
            "StringList",
            make_record_type(vec![
                ("items", Type::Array(Box::new(Type::String)), true),
                ("count", Type::Integer, false),
            ]),
        )
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}

#[test]
fn test_generated_optional_type_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type(
            "WithOptional",
            make_record_type(vec![
                ("required_field", Type::String, true),
                ("optional_field", Type::Optional(Box::new(Type::Integer)), false),
            ]),
        )
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}

#[test]
fn test_generated_map_type_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type(
            "WithMap",
            make_record_type(vec![
                ("labels", Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }, true),
                ("annotations", Type::Map { key: Box::new(Type::String), value: Box::new(Type::String) }, false),
            ]),
        )
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}

#[test]
fn test_generated_dyn_type_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type(
            "WithDyn",
            make_record_type(vec![
                ("data", Type::Any, true),
                ("metadata", Type::Any, false),
            ]),
        )
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}

#[test]
fn test_generated_multiple_types_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type("TypeA", Type::String)
        .add_type("TypeB", Type::Integer)
        .add_type(
            "TypeC",
            make_record_type(vec![
                ("a", Type::String, true),
                ("b", Type::Integer, false),
            ]),
        )
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}

#[test]
fn test_generated_reserved_keyword_fields_is_valid_nickel() {
    if !nickel_available() {
        eprintln!("Skipping test: nickel not available");
        return;
    }

    // Test that reserved keywords as field names are properly escaped
    let ir = IRBuilder::new()
        .module("test.v1")
        .add_type(
            "WithReservedKeywords",
            make_record_type(vec![
                ("type", Type::String, true),     // Reserved keyword
                ("if", Type::String, false),      // Reserved keyword
                ("let", Type::String, false),     // Reserved keyword
                ("$ref", Type::String, false),    // Starts with $
                ("normal_field", Type::String, false),
            ]),
        )
        .build();

    let registry = Arc::new(ModuleRegistry::new());
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir).expect("Failed to generate Nickel");

    if let Err(e) = nickel_check(&output) {
        panic!("Generated Nickel is invalid:\n{}\n\nGenerated code:\n{}", e, output);
    }
}
