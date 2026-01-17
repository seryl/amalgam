//! Tests for comprehensive validation generation in Nickel output
//!
//! Verifies that all ValidationRules fields are properly translated
//! to Nickel contract expressions.

use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::{
    ir::{Module, TypeDefinition, IR},
    module_registry::ModuleRegistry,
    types::{ContractRule, Field, StringFormat, Type, ValidationRules},
};
use std::collections::BTreeMap;
use std::sync::Arc;

fn create_test_ir(types: Vec<TypeDefinition>) -> IR {
    IR {
        modules: vec![Module {
            name: "test.v1".to_string(),
            imports: Vec::new(),
            types,
            constants: Vec::new(),
            metadata: Default::default(),
        }],
    }
}

#[test]
fn test_exclusive_bounds_generation() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "score".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::Number),
                            constraints: ValidationRules {
                                exclusive_minimum: Some(0.0),
                                exclusive_maximum: Some(100.0),
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("Score between 0 and 100 exclusive".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify exclusive bounds are generated
    assert!(
        output.contains("value > 0"),
        "Should contain exclusive minimum check. Output:\n{}",
        output
    );
    assert!(
        output.contains("value < 100"),
        "Should contain exclusive maximum check. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_array_constraints_generation() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "tags".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::Array(Box::new(Type::String))),
                            constraints: ValidationRules {
                                min_items: Some(1),
                                max_items: Some(10),
                                unique_items: Some(true),
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("1-10 unique tags".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify array constraints are generated
    assert!(
        output.contains("std.array.length value >= 1"),
        "Should contain min_items check. Output:\n{}",
        output
    );
    assert!(
        output.contains("std.array.length value <= 10"),
        "Should contain max_items check. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_enum_values_generation() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "status".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::String),
                            constraints: ValidationRules {
                                allowed_values: Some(vec![
                                    serde_json::Value::String("pending".to_string()),
                                    serde_json::Value::String("running".to_string()),
                                    serde_json::Value::String("completed".to_string()),
                                ]),
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("Status enum".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify enum validation is generated
    assert!(
        output.contains("std.array.elem value"),
        "Should contain enum membership check. Output:\n{}",
        output
    );
    assert!(
        output.contains("\"pending\"") && output.contains("\"running\"") && output.contains("\"completed\""),
        "Should contain all enum values. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_string_format_generation() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "email".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::String),
                            constraints: ValidationRules {
                                format: Some(StringFormat::Email),
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("Email address".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields.insert(
                    "ip".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::String),
                            constraints: ValidationRules {
                                format: Some(StringFormat::Ipv4),
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("IPv4 address".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify format validators are generated
    assert!(
        output.contains("std.string.is_match"),
        "Should contain format regex validation. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_k8s_format_generation() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "name".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::String),
                            constraints: ValidationRules {
                                format: Some(StringFormat::Dns1123Subdomain),
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("DNS-1123 subdomain name".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify K8s DNS format validator includes length constraint
    assert!(
        output.contains("std.string.length value <= 253"),
        "Should contain DNS-1123 length constraint. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_field_level_validation_propagation() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "port".to_string(),
                    Field {
                        ty: Type::Integer,
                        required: true,
                        description: Some("Port number".to_string()),
                        default: None,
                        // Field-level validation
                        validation: Some(ValidationRules {
                            minimum: Some(1.0),
                            maximum: Some(65535.0),
                            ..Default::default()
                        }),
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify field-level validation is applied
    assert!(
        output.contains("value >= 1") && output.contains("value <= 65535"),
        "Should contain field-level validation. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_field_level_contracts_propagation() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "percentage".to_string(),
                    Field {
                        ty: Type::Number,
                        required: true,
                        description: Some("Percentage value".to_string()),
                        default: None,
                        validation: None,
                        // Field-level contract
                        contracts: vec![ContractRule {
                            name: "percentage_range".to_string(),
                            expression: "value >= 0 && value <= 100".to_string(),
                            description: Some("Must be 0-100".to_string()),
                            error_message: Some("Percentage must be between 0 and 100".to_string()),
                        }],
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify contract is generated (with std.contract.from_predicate for error message)
    assert!(
        output.contains("value >= 0 && value <= 100"),
        "Should contain contract expression. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_k8s_embedded_resource_structure() -> Result<(), Box<dyn std::error::Error>> {
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "embeddedResource".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::Record {
                                fields: BTreeMap::new(),
                                open: true,
                            }),
                            constraints: ValidationRules {
                                k8s_embedded_resource: true,
                                ..Default::default()
                            },
                        },
                        required: false,
                        description: Some("Embedded K8s resource".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify embedded resource has proper structure, not just Dyn
    assert!(
        output.contains("apiVersion") && output.contains("kind"),
        "Should contain K8s resource structure (apiVersion, kind). Output:\n{}",
        output
    );
    assert!(
        !output.contains("| Dyn\n"),
        "Should NOT be bare Dyn (should have structure). Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_int_or_string_fqn_coercion() -> Result<(), Box<dyn std::error::Error>> {
    // Test that FQN references to IntOrString are coerced to proper union contracts
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "port".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "io.k8s.apimachinery.pkg.util.intstr.IntOrString".to_string(),
                            module: None,
                        },
                        required: true,
                        description: Some("Port number or name".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify IntOrString is coerced to a proper union contract, not left as FQN
    assert!(
        !output.contains("io.k8s.apimachinery.pkg.util.intstr.IntOrString"),
        "Should NOT contain raw IntOrString FQN. Output:\n{}",
        output
    );
    assert!(
        output.contains("std.is_number value || std.is_string value"),
        "Should contain IntOrString union predicate. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_cel_validation_translation() -> Result<(), Box<dyn std::error::Error>> {
    // Test that K8s CEL validations are translated to Nickel contracts
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "items".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::Array(Box::new(Type::String))),
                            constraints: ValidationRules {
                                k8s_cel_validations: vec![
                                    "self.size() > 0".to_string(),
                                    "self.size() <= 100".to_string(),
                                ],
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("Non-empty array with max 100 items".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify CEL validations are translated to Nickel
    assert!(
        output.contains("std.array.length value > 0"),
        "Should contain translated size() > 0 check. Output:\n{}",
        output
    );
    assert!(
        output.contains("std.array.length value <= 100"),
        "Should contain translated size() <= 100 check. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_cel_string_validation_translation() -> Result<(), Box<dyn std::error::Error>> {
    // Test that string-related CEL validations are translated
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "name".to_string(),
                    Field {
                        ty: Type::Constrained {
                            base_type: Box::new(Type::String),
                            constraints: ValidationRules {
                                k8s_cel_validations: vec![
                                    "self.startsWith(\"prefix-\")".to_string(),
                                ],
                                ..Default::default()
                            },
                        },
                        required: true,
                        description: Some("Name with prefix".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify string CEL validations are translated
    assert!(
        output.contains("std.string.is_match") && output.contains("^prefix-"),
        "Should contain translated startsWith check. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_contract_rules_generation() -> Result<(), Box<dyn std::error::Error>> {
    // Test that ContractRules are properly generated
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "value".to_string(),
                    Field {
                        ty: Type::String,
                        required: true,
                        description: Some("A value with contract".to_string()),
                        default: None,
                        validation: None,
                        contracts: vec![ContractRule {
                            name: "oneOf".to_string(),
                            expression: "(std.is_string value || std.is_number value)".to_string(),
                            description: Some("Value must match one of the types".to_string()),
                            error_message: Some("Value type mismatch".to_string()),
                        }],
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify contract is generated
    assert!(
        output.contains("std.is_string value || std.is_number value"),
        "Should contain the contract expression. Output:\n{}",
        output
    );
    assert!(
        output.contains("std.contract.from_predicate"),
        "Should use from_predicate for contracts with error messages. Output:\n{}",
        output
    );

    Ok(())
}

#[test]
fn test_quantity_fqn_coercion() -> Result<(), Box<dyn std::error::Error>> {
    // Test that FQN references to Quantity are coerced to String
    let ir = create_test_ir(vec![TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "storage".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "io.k8s.apimachinery.pkg.api.resource.Quantity".to_string(),
                            module: None,
                        },
                        required: true,
                        description: Some("Storage capacity".to_string()),
                        default: None,
                        validation: None,
                        contracts: Vec::new(),
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    }]);

    let registry = Arc::new(ModuleRegistry::from_ir(&ir));
    let mut codegen = NickelCodegen::new(registry);
    let output = codegen.generate(&ir)?;

    // Verify Quantity is coerced to String, not left as FQN
    assert!(
        !output.contains("io.k8s.apimachinery.pkg.api.resource.Quantity"),
        "Should NOT contain raw Quantity FQN. Output:\n{}",
        output
    );
    // The storage field should be typed as String
    assert!(
        output.contains("| String"),
        "Storage field should be typed as String. Output:\n{}",
        output
    );

    Ok(())
}
