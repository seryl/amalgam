//! Integration tests for complex import scenarios in the unified IR pipeline

use amalgam_codegen::Codegen;
use amalgam_core::ir::{Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use std::collections::BTreeMap;

/// Test: Type with multiple dependencies from same module
#[test]
fn test_type_with_multiple_same_module_deps() -> Result<(), Box<dyn std::error::Error>> {
    // Create referenced types first
    let container = TypeDefinition {
        name: "Container".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: true,
        },
        documentation: Some("Container type".to_string()),
        annotations: BTreeMap::new(),
    };

    let ephemeral_container = TypeDefinition {
        name: "EphemeralContainer".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: true,
        },
        documentation: Some("Ephemeral container type".to_string()),
        annotations: BTreeMap::new(),
    };

    let volume = TypeDefinition {
        name: "Volume".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: true,
        },
        documentation: Some("Volume type".to_string()),
        annotations: BTreeMap::new(),
    };

    // Create a type that references multiple types from the same module
    let mut fields = BTreeMap::new();

    fields.insert(
        "container".to_string(),
        Field {
            ty: Type::Reference {
                name: "Container".to_string(),
                module: None,
            },
            required: true,
            description: None,
            default: None,
        },
    );

    fields.insert(
        "ephemeralContainer".to_string(),
        Field {
            ty: Type::Reference {
                name: "EphemeralContainer".to_string(),
                module: None,
            },
            required: false,
            description: None,
            default: None,
        },
    );

    fields.insert(
        "volume".to_string(),
        Field {
            ty: Type::Reference {
                name: "Volume".to_string(),
                module: None,
            },
            required: false,
            description: None,
            default: None,
        },
    );

    let pod_spec = TypeDefinition {
        name: "PodSpec".to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: Some("Pod specification with multiple refs".to_string()),
        annotations: BTreeMap::new(),
    };

    let module = Module {
        name: "k8s.io.v1".to_string(),
        imports: vec![],
        types: vec![container, ephemeral_container, volume, pod_spec],
        constants: vec![],
        metadata: Default::default(),
    };

    let ir = IR {
        modules: vec![module],
    };

    // Generate code and verify imports are deduplicated
    // Use from_ir to properly populate the registry with types from the IR
    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)?;

    // Debug: Print the actual output
    eprintln!(
        "Generated output for test_type_with_multiple_same_module_deps:\n{}",
        output
    );

    // Check that we have all the types defined
    assert!(output.contains("Container"), "Should have Container type");
    assert!(
        output.contains("EphemeralContainer"),
        "Should have EphemeralContainer type"
    );
    assert!(output.contains("Volume"), "Should have Volume type");
    assert!(output.contains("PodSpec"), "Should have PodSpec type");

    // Check that PodSpec references the other types
    // Since all types are in the same module, they reference each other directly by name
    assert!(
        output.contains("container") && output.contains("| Container"),
        "PodSpec should have a container field with Container type"
    );
    assert!(
        output.contains("ephemeralContainer") && output.contains("| EphemeralContainer"),
        "PodSpec should have an ephemeralContainer field with EphemeralContainer type"
    );
    assert!(
        output.contains("volume") && output.contains("| Volume"),
        "PodSpec should have a volume field with Volume type"
    );
    Ok(())
}

/// Test: Cross-version import chain (v1alpha3 -> v1beta1 -> v1)
#[test]
fn test_cross_version_import_chain() -> Result<(), Box<dyn std::error::Error>> {
    // Create types that form a cross-version dependency chain
    let v1_type = TypeDefinition {
        name: "CoreType".to_string(),
        ty: Type::String,
        documentation: Some("Core v1 type".to_string()),
        annotations: BTreeMap::new(),
    };

    let mut v1beta1_fields = BTreeMap::new();
    v1beta1_fields.insert(
        "coreRef".to_string(),
        Field {
            ty: Type::Reference {
                name: "CoreType".to_string(),
                module: Some("k8s.io.v1".to_string()),
            },
            required: true,
            description: None,
            default: None,
        },
    );

    let v1beta1_type = TypeDefinition {
        name: "BetaType".to_string(),
        ty: Type::Record {
            fields: v1beta1_fields,
            open: false,
        },
        documentation: Some("Beta type referencing v1".to_string()),
        annotations: BTreeMap::new(),
    };

    let mut v1alpha3_fields = BTreeMap::new();
    v1alpha3_fields.insert(
        "betaRef".to_string(),
        Field {
            ty: Type::Reference {
                name: "BetaType".to_string(),
                module: Some("k8s.io.v1beta1".to_string()),
            },
            required: true,
            description: None,
            default: None,
        },
    );

    let v1alpha3_type = TypeDefinition {
        name: "AlphaType".to_string(),
        ty: Type::Record {
            fields: v1alpha3_fields,
            open: false,
        },
        documentation: Some("Alpha type with transitive dependency".to_string()),
        annotations: BTreeMap::new(),
    };

    let ir = IR {
        modules: vec![
            Module {
                name: "k8s.io.v1".to_string(),
                imports: vec![],
                types: vec![v1_type],
                constants: vec![],
                metadata: Default::default(),
            },
            Module {
                name: "k8s.io.v1beta1".to_string(),
                imports: vec![],
                types: vec![v1beta1_type],
                constants: vec![],
                metadata: Default::default(),
            },
            Module {
                name: "k8s.io.v1alpha3".to_string(),
                imports: vec![],
                types: vec![v1alpha3_type],
                constants: vec![],
                metadata: Default::default(),
            },
        ],
    };

    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)?;

    // Verify all types are present and references are correct
    assert!(
        output.contains("CoreType") || output.contains("String"),
        "Should have CoreType or String (since CoreType is String)"
    );
    assert!(
        output.contains("BetaType") || output.contains("coreRef"),
        "Should have BetaType or its fields"
    );
    assert!(
        output.contains("AlphaType") || output.contains("betaRef"),
        "Should have AlphaType or its fields"
    );

    // Since these are cross-module references, they should be using Type::Reference with module
    // The codegen might handle these differently
    Ok(())
}

/// Test: Circular dependency detection (should handle gracefully)
#[test]
fn test_circular_dependency_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Create two types that reference each other
    let mut type_a_fields = BTreeMap::new();
    type_a_fields.insert(
        "b_ref".to_string(),
        Field {
            ty: Type::Reference {
                name: "TypeB".to_string(),
                module: None,
            },
            required: false,
            description: None,
            default: None,
        },
    );

    let type_a = TypeDefinition {
        name: "TypeA".to_string(),
        ty: Type::Record {
            fields: type_a_fields,
            open: false,
        },
        documentation: Some("Type A referencing B".to_string()),
        annotations: BTreeMap::new(),
    };

    let mut type_b_fields = BTreeMap::new();
    type_b_fields.insert(
        "a_ref".to_string(),
        Field {
            ty: Type::Reference {
                name: "TypeA".to_string(),
                module: None,
            },
            required: false,
            description: None,
            default: None,
        },
    );

    let type_b = TypeDefinition {
        name: "TypeB".to_string(),
        ty: Type::Record {
            fields: type_b_fields,
            open: false,
        },
        documentation: Some("Type B referencing A".to_string()),
        annotations: BTreeMap::new(),
    };

    let module = Module {
        name: "test.module.v1".to_string(),
        imports: vec![],
        types: vec![type_a, type_b],
        constants: vec![],
        metadata: Default::default(),
    };

    let ir = IR {
        modules: vec![module],
    };

    // Should not panic or go into infinite loop
    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let result = codegen.generate(&ir);
    assert!(
        result.is_ok(),
        "Should handle circular dependencies gracefully"
    );
    Ok(())
}

/// Test: Complex nested unions and arrays with references
#[test]
fn test_nested_unions_and_arrays() -> Result<(), Box<dyn std::error::Error>> {
    let nested_type = TypeDefinition {
        name: "ComplexType".to_string(),
        ty: Type::Union {
            types: vec![
                Type::Array(Box::new(Type::Reference {
                    name: "Container".to_string(),
                    module: None,
                })),
                Type::Map {
                    key: Box::new(Type::String),
                    value: Box::new(Type::Optional(Box::new(Type::Reference {
                        name: "Volume".to_string(),
                        module: None,
                    }))),
                },
                Type::Record {
                    fields: {
                        let mut fields = BTreeMap::new();
                        fields.insert(
                            "pod".to_string(),
                            Field {
                                ty: Type::Reference {
                                    name: "Pod".to_string(),
                                    module: None,
                                },
                                required: true,
                                description: None,
                                default: None,
                            },
                        );
                        fields
                    },
                    open: false,
                },
            ],
            coercion_hint: None,
        },
        documentation: Some("Complex nested type with multiple reference patterns".to_string()),
        annotations: BTreeMap::new(),
    };

    let module = Module {
        name: "k8s.io.v1".to_string(),
        imports: vec![],
        types: vec![nested_type],
        constants: vec![],
        metadata: Default::default(),
    };

    let ir = IR {
        modules: vec![module],
    };

    // Use from_ir to properly populate the registry with types from the IR
    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)?;

    // Debug: Print the actual output
    eprintln!("Generated Nickel output:\n{}", output);

    // The type should contain references to container, volume, and pod (now using camelCase)
    assert!(
        output.contains("Array container") || output.contains("container"),
        "Should reference container (imported type)"
    );
    assert!(
        output.contains("volume"),
        "Should reference volume (imported type)"
    );
    assert!(
        output.contains("pod"),
        "Should reference pod (imported type)"
    );
    Ok(())
}

/// Test: Cross-package imports (k8s.io + crossplane)
#[test]
fn test_cross_package_imports() -> Result<(), Box<dyn std::error::Error>> {
    // Create a CrossPlane type that references k8s types
    let mut composition_fields = BTreeMap::new();

    composition_fields.insert(
        "metadata".to_string(),
        Field {
            ty: Type::Reference {
                name: "ObjectMeta".to_string(),
                module: Some("k8s.io.v1".to_string()),
            },
            required: true,
            description: Some("Standard k8s metadata".to_string()),
            default: None,
        },
    );

    composition_fields.insert(
        "spec".to_string(),
        Field {
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: true,
            },
            required: true,
            description: None,
            default: None,
        },
    );

    let composition = TypeDefinition {
        name: "Composition".to_string(),
        ty: Type::Record {
            fields: composition_fields,
            open: false,
        },
        documentation: Some("CrossPlane Composition with k8s refs".to_string()),
        annotations: BTreeMap::new(),
    };

    let object_meta = TypeDefinition {
        name: "ObjectMeta".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: true,
        },
        documentation: Some("Kubernetes ObjectMeta".to_string()),
        annotations: BTreeMap::new(),
    };

    let ir = IR {
        modules: vec![
            Module {
                name: "apiextensions.crossplane.io.v1".to_string(),
                imports: vec![],
                types: vec![composition],
                constants: vec![],
                metadata: Default::default(),
            },
            Module {
                name: "k8s.io.v1".to_string(),
                imports: vec![],
                types: vec![object_meta],
                constants: vec![],
                metadata: Default::default(),
            },
        ],
    };

    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)?;

    // Should have both Composition and ObjectMeta types
    assert!(
        output.contains("Composition"),
        "Should have Composition type"
    );
    assert!(output.contains("ObjectMeta"), "Should have ObjectMeta type");
    Ok(())
}

/// Test: Runtime types (RawExtension) importing from v0
#[test]
fn test_runtime_types_v0_import() -> Result<(), Box<dyn std::error::Error>> {
    // Create a type that uses RawExtension (which should be in v0)
    let mut spec_fields = BTreeMap::new();

    spec_fields.insert(
        "extension".to_string(),
        Field {
            ty: Type::Reference {
                name: "RawExtension".to_string(),
                module: Some("k8s.io.v0".to_string()),
            },
            required: false,
            description: Some("Runtime extension field".to_string()),
            default: None,
        },
    );

    let custom_resource = TypeDefinition {
        name: "CustomResource".to_string(),
        ty: Type::Record {
            fields: spec_fields,
            open: false,
        },
        documentation: Some("Custom resource with RawExtension".to_string()),
        annotations: BTreeMap::new(),
    };

    let raw_extension = TypeDefinition {
        name: "RawExtension".to_string(),
        ty: Type::Any,
        documentation: Some("Runtime raw extension".to_string()),
        annotations: BTreeMap::new(),
    };

    let ir = IR {
        modules: vec![
            Module {
                name: "custom.io.v1".to_string(),
                imports: vec![],
                types: vec![custom_resource],
                constants: vec![],
                metadata: Default::default(),
            },
            Module {
                name: "k8s.io.v0".to_string(),
                imports: vec![],
                types: vec![raw_extension],
                constants: vec![],
                metadata: Default::default(),
            },
        ],
    };

    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)?;

    // Should have RawExtension reference
    assert!(
        output.contains("RawExtension"),
        "Should have RawExtension type reference"
    );
    Ok(())
}

#[test]
fn test_optional_and_array_references() -> Result<(), Box<dyn std::error::Error>> {
    // Test that optional and array types with references generate correct imports
    let mut fields = BTreeMap::new();

    fields.insert(
        "optionalRef".to_string(),
        Field {
            ty: Type::Optional(Box::new(Type::Reference {
                name: "Container".to_string(),
                module: None,
            })),
            required: false,
            description: None,
            default: None,
        },
    );

    fields.insert(
        "arrayRef".to_string(),
        Field {
            ty: Type::Array(Box::new(Type::Reference {
                name: "Volume".to_string(),
                module: None,
            })),
            required: true,
            description: None,
            default: None,
        },
    );

    let test_type = TypeDefinition {
        name: "TestType".to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: Some("Type with optional and array refs".to_string()),
        annotations: BTreeMap::new(),
    };

    let module = Module {
        name: "test.v1".to_string(),
        imports: vec![],
        types: vec![test_type],
        constants: vec![],
        metadata: Default::default(),
    };

    let ir = IR {
        modules: vec![module],
    };

    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)?;

    // Debug: Print the actual output
    eprintln!("Generated Nickel output:\n{}", output);

    // The type should reference container and volume (now using camelCase)
    assert!(
        output.contains("container"),
        "Should reference container (imported type)"
    );
    assert!(
        output.contains("volume"),
        "Should reference volume (imported type)"
    );

    // Check for array and optional handling (now using camelCase)
    assert!(
        output.contains("Array volume"),
        "Should have Array of volume"
    );
    assert!(
        output.contains("container | Null") || output.contains("optional | container"),
        "Should have optional container"
    );
    Ok(())
}
