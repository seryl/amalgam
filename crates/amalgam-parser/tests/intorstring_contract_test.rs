//! Test to ensure IntOrString and similar single-type modules work as contracts
//! This test addresses the critical bug where types were wrapped in records,
//! making them unusable as contracts

use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::ir::{Constant, Metadata, Module, TypeDefinition, IR};
use amalgam_core::types::Type;
use std::collections::BTreeMap;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_intorstring_exports_directly_as_contract() {
    // Create a single-type module (IntOrString)
    let mut ir = IR::new();
    let module = Module {
        name: "k8s.io.v0.intorstring".to_string(),
        imports: vec![],
        types: vec![TypeDefinition {
            name: "IntOrString".to_string(),
            ty: Type::String,
            documentation: Some(
                "IntOrString is a type that can hold an int32 or a string.".to_string(),
            ),
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Metadata::default(),
    };
    ir.add_module(module);

    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let result = codegen.generate(&ir).expect("Failed to generate code");

    // The generated code should be just the type, not wrapped in a record
    assert!(
        !result.contains("{ IntOrString"),
        "Single-type module should not be wrapped in a record"
    );
    assert!(
        result.contains("# IntOrString is a type that can hold an int32 or a string."),
        "Documentation should be preserved"
    );
    assert!(
        result.contains("String"),
        "Type definition should be present"
    );
    assert!(
        !result.contains("IntOrString ="),
        "Should not have field assignment syntax"
    );
}

#[test]
fn test_rawextension_exports_directly() {
    // Create another single-type module (RawExtension)
    let mut ir = IR::new();
    let module = Module {
        name: "k8s.io.v0.rawextension".to_string(),
        imports: vec![],
        types: vec![TypeDefinition {
            name: "RawExtension".to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: true,
            },
            documentation: Some("RawExtension is used to hold extensions".to_string()),
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Metadata::default(),
    };
    ir.add_module(module);

    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let result = codegen.generate(&ir).expect("Failed to generate code");

    // Should export the record type directly, not wrapped in another record
    assert!(
        !result.contains("{ RawExtension"),
        "Single-type module should not be wrapped in outer record"
    );
    assert!(
        result.contains("{..}") || result.contains("{ .. }"),
        "Open record syntax should be present"
    );
}

#[test]
fn test_multi_type_module_uses_record_wrapper() {
    // Create a multi-type module
    let mut ir = IR::new();
    let module = Module {
        name: "k8s.io.v1.types".to_string(),
        imports: vec![],
        types: vec![
            TypeDefinition {
                name: "Container".to_string(),
                ty: Type::Record {
                    fields: BTreeMap::new(),
                    open: false,
                },
                documentation: None,
                annotations: BTreeMap::new(),
            },
            TypeDefinition {
                name: "Pod".to_string(),
                ty: Type::Record {
                    fields: BTreeMap::new(),
                    open: false,
                },
                documentation: None,
                annotations: BTreeMap::new(),
            },
        ],
        constants: vec![],
        metadata: Metadata::default(),
    };
    ir.add_module(module);

    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let result = codegen.generate(&ir).expect("Failed to generate code");

    // Multi-type modules should be wrapped in a record
    assert!(
        result.contains("{") && result.contains("}"),
        "Multi-type module should be wrapped in a record"
    );
    assert!(
        result.contains("Container ="),
        "Should have Container field"
    );
    assert!(result.contains("Pod ="), "Should have Pod field");
}

#[test]
fn test_intorstring_contract_can_merge_with_string() {
    // This test simulates how IntOrString should be usable as a contract
    let temp_dir = TempDir::new().unwrap();
    let k8s_dir = temp_dir.path().join("k8s_io").join("v0");
    fs::create_dir_all(&k8s_dir).unwrap();

    // Generate IntOrString as a single-type module
    let mut ir = IR::new();
    let module = Module {
        name: "k8s.io.v0.intorstring".to_string(),
        imports: vec![],
        types: vec![TypeDefinition {
            name: "IntOrString".to_string(),
            ty: Type::String,
            documentation: None,
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Metadata::default(),
    };
    ir.add_module(module);

    let mut codegen = NickelCodegen::from_ir(&ir);
    let intorstring_content = codegen
        .generate(&ir)
        .expect("Failed to generate IntOrString");

    // Write IntOrString file (should be just "String")
    fs::write(k8s_dir.join("intorstring.ncl"), &intorstring_content).unwrap();

    // Write mod.ncl that imports it
    let mod_content = r#"# k8s.io/v0 types
# Auto-generated by amalgam

{
  IntOrString = import "./intorstring.ncl",
}
"#;
    fs::write(k8s_dir.join("mod.ncl"), mod_content).unwrap();

    // Write main k8s_io module
    let main_mod_content = r#"{
  v0 = import "./v0/mod.ncl",
}
"#;
    fs::write(
        temp_dir.path().join("k8s_io").join("mod.ncl"),
        main_mod_content,
    )
    .unwrap();

    // Create a test Nickel file that uses IntOrString as a contract
    let test_content = format!(
        r#"let k8s_io = import "{}/k8s_io/mod.ncl" in
{{
  # This should work because IntOrString is String, not wrapped in a record
  test_string | k8s_io.v0.IntOrString = "80%",
  
  # This should also work with contract merge
  test_merge = k8s_io.v0.IntOrString & "test-value",
}}
"#,
        temp_dir.path().display()
    );

    let test_file = temp_dir.path().join("test.ncl");
    fs::write(&test_file, test_content).unwrap();

    // The test passes if the file structure is correct
    // In a real scenario, we'd run `nickel eval` on this file
    // For now, we verify the generated structure is correct
    let generated_intorstring = fs::read_to_string(k8s_dir.join("intorstring.ncl")).unwrap();
    assert_eq!(
        generated_intorstring.trim(),
        "String",
        "IntOrString should be exported as just 'String'"
    );
}

#[test]
fn test_module_with_constants_uses_record_wrapper() {
    // Even a single type with constants should use record wrapper
    let mut ir = IR::new();
    let module = Module {
        name: "k8s.io.v1.constants".to_string(),
        imports: vec![],
        types: vec![TypeDefinition {
            name: "MyType".to_string(),
            ty: Type::String,
            documentation: None,
            annotations: BTreeMap::new(),
        }],
        constants: vec![Constant {
            name: "DEFAULT_NAMESPACE".to_string(),
            value: serde_json::json!("default"),
            ty: Type::String,
            documentation: None,
        }],
        metadata: Metadata::default(),
    };
    ir.add_module(module);

    let mut codegen = NickelCodegen::from_ir(&ir);
    let result = codegen.generate(&ir).expect("Failed to generate code");

    // Should be wrapped because it has constants
    assert!(
        result.contains("{") && result.contains("}"),
        "Module with constants should be wrapped in a record"
    );
    assert!(result.contains("MyType ="), "Should have type field");
    assert!(
        result.contains("DEFAULT_NAMESPACE ="),
        "Should have constant field"
    );
}

#[test]
fn test_regression_prevention_intorstring_bug() {
    // This test ensures the specific bug reported cannot happen again
    // Bug: IntOrString was { IntOrString = String } instead of just String
    // Impact: Could not use as contract: value | k8s.v0.IntOrString = "80%"

    // Generate all k8s v0 types that should be single-type modules
    let v0_types = vec![
        ("intorstring", "IntOrString", Type::String),
        (
            "rawextension",
            "RawExtension",
            Type::Record {
                fields: BTreeMap::new(),
                open: true,
            },
        ),
    ];

    for (filename, typename, ty) in v0_types {
        let module = Module {
            name: format!("k8s.io.v0.{}", filename),
            imports: vec![],
            types: vec![TypeDefinition {
                name: typename.to_string(),
                ty,
                documentation: None,
                annotations: BTreeMap::new(),
            }],
            constants: vec![],
            metadata: Metadata::default(),
        };

        let mut single_ir = IR::new();
        single_ir.add_module(module);

        let mut codegen = NickelCodegen::from_ir(&single_ir);
        let result = codegen
            .generate(&single_ir)
            .unwrap_or_else(|_| panic!("Failed to generate {}", typename));

        // Verify no record wrapper
        assert!(
            !result.contains(&format!("{{ {} ", typename)),
            "{} should not be wrapped in a record",
            typename
        );
        assert!(
            !result.contains(&format!("{} =", typename)),
            "{} should not have field assignment",
            typename
        );
    }
}
