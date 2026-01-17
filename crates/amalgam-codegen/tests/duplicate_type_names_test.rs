//! Tests for duplicate type name handling
//!
//! Verifies that the codegen correctly handles cases where both a resource type
//! and its list type have the same name (e.g., CronJob and CronJobList both named "CronJob")

use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::{
    ir::{Module, TypeDefinition, IR},
    module_registry::ModuleRegistry,
    types::{Field, Type},
};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Creates a K8s-style list type (has apiVersion, kind, items array, metadata)
fn create_list_type(item_type_name: &str) -> Type {
    let mut fields = BTreeMap::new();

    fields.insert(
        "apiVersion".to_string(),
        Field {
            ty: Type::String,
            required: false,
            description: Some("APIVersion".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    fields.insert(
        "kind".to_string(),
        Field {
            ty: Type::String,
            required: false,
            description: Some("Kind".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    fields.insert(
        "items".to_string(),
        Field {
            ty: Type::Array(Box::new(Type::Reference {
                name: item_type_name.to_string(),
                module: None,
            })),
            required: true,
            description: Some("List of items".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
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
            description: Some("List metadata".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    Type::Record { fields, open: false }
}

/// Creates a simple K8s resource type
fn create_resource_type() -> Type {
    let mut fields = BTreeMap::new();

    fields.insert(
        "apiVersion".to_string(),
        Field {
            ty: Type::String,
            required: false,
            description: Some("APIVersion".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    fields.insert(
        "kind".to_string(),
        Field {
            ty: Type::String,
            required: false,
            description: Some("Kind".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
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
            description: Some("Object metadata".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
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
            description: Some("Spec".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    Type::Record { fields, open: false }
}

#[test]
fn test_duplicate_type_names_are_deduplicated() -> Result<(), Box<dyn std::error::Error>> {
    // Create an IR with duplicate type names (CronJob for both resource and list)
    let ir = IR {
        modules: vec![Module {
            name: "batch.v1".to_string(),
            imports: Vec::new(),
            types: vec![
                // First CronJob - the actual resource type
                TypeDefinition {
                    name: "CronJob".to_string(),
                    ty: create_resource_type(),
                    documentation: Some("CronJob resource".to_string()),
                    annotations: BTreeMap::new(),
                },
                // Second CronJob - actually CronJobList but with wrong name
                TypeDefinition {
                    name: "CronJob".to_string(),
                    ty: create_list_type("CronJob"),
                    documentation: Some("CronJob list".to_string()),
                    annotations: BTreeMap::new(),
                },
            ],
            constants: Vec::new(),
            metadata: Default::default(),
        }],
    };

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify that we have both CronJob and CronJobList in output
    assert!(
        output.contains("CronJob ="),
        "Output should contain CronJob type. Got:\n{}",
        output
    );
    assert!(
        output.contains("CronJobList ="),
        "Output should contain CronJobList type (renamed from duplicate CronJob). Got:\n{}",
        output
    );

    // Verify there's exactly one "CronJob =" (not counting CronJobList)
    let cronjob_count = output.matches("CronJob =").count();
    let cronjoblist_count = output.matches("CronJobList =").count();

    assert_eq!(
        cronjob_count, 1,
        "Should have exactly one CronJob type definition. Found {}. Output:\n{}",
        cronjob_count, output
    );
    assert_eq!(
        cronjoblist_count, 1,
        "Should have exactly one CronJobList type definition. Found {}. Output:\n{}",
        cronjoblist_count, output
    );

    Ok(())
}

#[test]
fn test_multiple_duplicate_types_are_deduplicated() -> Result<(), Box<dyn std::error::Error>> {
    // Test with multiple duplicate type pairs
    let ir = IR {
        modules: vec![Module {
            name: "batch.v1".to_string(),
            imports: Vec::new(),
            types: vec![
                // CronJob pair
                TypeDefinition {
                    name: "CronJob".to_string(),
                    ty: create_resource_type(),
                    documentation: None,
                    annotations: BTreeMap::new(),
                },
                TypeDefinition {
                    name: "CronJob".to_string(),
                    ty: create_list_type("CronJob"),
                    documentation: None,
                    annotations: BTreeMap::new(),
                },
                // Job pair
                TypeDefinition {
                    name: "Job".to_string(),
                    ty: create_resource_type(),
                    documentation: None,
                    annotations: BTreeMap::new(),
                },
                TypeDefinition {
                    name: "Job".to_string(),
                    ty: create_list_type("Job"),
                    documentation: None,
                    annotations: BTreeMap::new(),
                },
            ],
            constants: Vec::new(),
            metadata: Default::default(),
        }],
    };

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify all four types are present with correct names
    assert!(output.contains("CronJob ="), "Should contain CronJob");
    assert!(output.contains("CronJobList ="), "Should contain CronJobList");
    assert!(output.contains("Job ="), "Should contain Job");
    assert!(output.contains("JobList ="), "Should contain JobList");

    Ok(())
}

#[test]
fn test_reserved_keyword_field_names_are_escaped() -> Result<(), Box<dyn std::error::Error>> {
    // Create a type with reserved keyword field names
    let mut fields = BTreeMap::new();

    // "type" is a Nickel reserved keyword
    fields.insert(
        "type".to_string(),
        Field {
            ty: Type::String,
            required: true,
            description: Some("Type of the condition".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    // "enum" is also reserved
    fields.insert(
        "enum".to_string(),
        Field {
            ty: Type::String,
            required: false,
            description: Some("Enum value".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    // Normal field for comparison
    fields.insert(
        "name".to_string(),
        Field {
            ty: Type::String,
            required: true,
            description: Some("Name".to_string()),
            default: None,
            validation: None,
            contracts: Vec::new(),
        },
    );

    let ir = IR {
        modules: vec![Module {
            name: "test.v1".to_string(),
            imports: Vec::new(),
            types: vec![TypeDefinition {
                name: "Condition".to_string(),
                ty: Type::Record { fields, open: false },
                documentation: None,
                annotations: BTreeMap::new(),
            }],
            constants: Vec::new(),
            metadata: Default::default(),
        }],
    };

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify reserved keywords are quoted
    assert!(
        output.contains("\"type\""),
        "Field 'type' should be quoted as \"type\". Got:\n{}",
        output
    );
    assert!(
        output.contains("\"enum\""),
        "Field 'enum' should be quoted as \"enum\". Got:\n{}",
        output
    );

    // Verify normal field is NOT quoted
    // (looking for 'name' without quotes, as a field definition)
    assert!(
        output.contains("\n    name\n") || output.contains("\n  name\n"),
        "Normal field 'name' should not be quoted. Got:\n{}",
        output
    );

    // Ensure we don't have type_field (the old incorrect behavior)
    assert!(
        !output.contains("type_field"),
        "Should not contain 'type_field' - should use quoted \"type\" instead. Got:\n{}",
        output
    );

    Ok(())
}
