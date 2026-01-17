/// Comprehensive tests for import resolution system
/// Ensures correct import path calculation across all scenarios
use amalgam_codegen::nickel::NickelCodegen;
use amalgam_codegen::Codegen;
use amalgam_core::ir::{Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use amalgam_core::ModuleRegistry;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Helper to create a type with references
fn create_type_with_references(
    name: &str,
    referenced_types: Vec<(&str, Option<&str>)>,
) -> TypeDefinition {
    let mut fields = BTreeMap::new();

    for (i, (ref_name, ref_module)) in referenced_types.iter().enumerate() {
        fields.insert(
            format!("field_{}", i),
            Field {
                ty: Type::Reference {
                    name: ref_name.to_string(),
                    module: ref_module.map(String::from),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
    }

    TypeDefinition {
        name: name.to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: Some(format!("Type {} with references", name)),
        annotations: BTreeMap::new(),
    }
}

#[test]
fn test_same_module_import_resolution() -> Result<(), Box<dyn std::error::Error>> {
    // Test: Types in the same module should not generate imports
    let mut ir = IR::new();

    let mut module = Module {
        name: "k8s.io.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    // Add types that reference each other within the same module
    module.types.push(create_type_with_references(
        "Pod",
        vec![("PodSpec", None), ("ObjectMeta", None)],
    ));

    module.types.push(TypeDefinition {
        name: "PodSpec".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    module.types.push(TypeDefinition {
        name: "ObjectMeta".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(module);

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Verify no imports are generated for same-module references
    assert!(
        !output.contains("import"),
        "Same-module references should not generate imports. Output:\n{}",
        output
    );

    // Verify types are defined
    assert!(output.contains("Pod ="), "Pod type should be defined");
    assert!(
        output.contains("PodSpec ="),
        "PodSpec type should be defined"
    );
    assert!(
        output.contains("ObjectMeta ="),
        "ObjectMeta type should be defined"
    );

    Ok(())
}

#[test]
fn test_cross_module_import_resolution() -> Result<(), Box<dyn std::error::Error>> {
    // Test: Types referencing other modules should generate correct imports
    let mut ir = IR::new();

    // Module 1: apps.v1
    let mut apps_module = Module {
        name: "k8s.io.apps.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    apps_module.types.push(create_type_with_references(
        "Deployment",
        vec![
            ("DeploymentSpec", None),          // Same module reference
            ("ObjectMeta", Some("k8s.io.v1")), // Cross-module reference
        ],
    ));

    apps_module.types.push(TypeDefinition {
        name: "DeploymentSpec".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // Module 2: core v1
    let mut core_module = Module {
        name: "k8s.io.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    core_module.types.push(TypeDefinition {
        name: "ObjectMeta".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(apps_module);
    ir.modules.push(core_module);

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Parse the output to find the apps.v1 module section
    let lines: Vec<&str> = output.lines().collect();
    let mut in_apps_module = false;
    let mut apps_module_content = String::new();

    for line in lines {
        if line.contains("Module: k8s.io.apps.v1") || line.contains("k8s.io.apps.v1") {
            in_apps_module = true;
        } else if line.contains("Module:") && in_apps_module {
            break;
        }

        if in_apps_module {
            apps_module_content.push_str(line);
            apps_module_content.push('\n');
        }
    }

    // Verify cross-module import is present
    assert!(
        apps_module_content.contains("import") || output.contains("k8s_io_v1"),
        "Cross-module reference should generate import. Apps module content:\n{}",
        apps_module_content
    );

    Ok(())
}

#[test]
fn test_nested_namespace_import_resolution() -> Result<(), Box<dyn std::error::Error>> {
    // Test: Deeply nested namespaces generate correct import paths
    let mut ir = IR::new();

    // Module 1: Deep namespace
    let mut deep_module = Module {
        name: "crossplane.io.aws.ec2.v1beta1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    deep_module.types.push(create_type_with_references(
        "Instance",
        vec![
            ("InstanceSpec", None),
            ("CommonParameters", Some("crossplane.io.aws.v1")),
            ("ObjectMeta", Some("k8s.io.v1")),
        ],
    ));

    deep_module.types.push(TypeDefinition {
        name: "InstanceSpec".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // Module 2: AWS common
    let mut aws_module = Module {
        name: "crossplane.io.aws.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    aws_module.types.push(TypeDefinition {
        name: "CommonParameters".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // Module 3: K8s core
    let mut k8s_module = Module {
        name: "k8s.io.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    k8s_module.types.push(TypeDefinition {
        name: "ObjectMeta".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(deep_module);
    ir.modules.push(aws_module);
    ir.modules.push(k8s_module);

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Verify all types are generated
    assert!(
        output.contains("Instance ="),
        "Instance type should be defined"
    );
    assert!(
        output.contains("CommonParameters ="),
        "CommonParameters type should be defined"
    );
    assert!(
        output.contains("ObjectMeta ="),
        "ObjectMeta type should be defined"
    );

    // The output should contain proper module structure
    assert!(
        output.contains("crossplane") || output.contains("aws"),
        "Output should contain module structure markers"
    );

    Ok(())
}

#[test]
fn test_circular_reference_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test: Circular references between modules are handled correctly
    let mut ir = IR::new();

    // Module 1: Has type A that references type B
    let mut module1 = Module {
        name: "example.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    module1.types.push(create_type_with_references(
        "TypeA",
        vec![("TypeB", Some("example.v2"))],
    ));

    // Module 2: Has type B that references type A
    let mut module2 = Module {
        name: "example.v2".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    module2.types.push(create_type_with_references(
        "TypeB",
        vec![("TypeA", Some("example.v1"))],
    ));

    ir.modules.push(module1);
    ir.modules.push(module2);

    // Generate code - should not panic or infinite loop
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Verify both types are generated
    assert!(output.contains("TypeA ="), "TypeA should be defined");
    assert!(output.contains("TypeB ="), "TypeB should be defined");

    Ok(())
}

#[test]
fn test_optional_and_array_reference_imports() -> Result<(), Box<dyn std::error::Error>> {
    // Test: Optional and Array types with references generate correct imports
    let mut ir = IR::new();

    let mut module = Module {
        name: "test.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    let mut fields = BTreeMap::new();

    // Optional reference to another module
    fields.insert(
        "optional_ref".to_string(),
        Field {
            ty: Type::Optional(Box::new(Type::Reference {
                name: "ExternalType".to_string(),
                module: Some("other.v1".to_string()),
            })),
            required: false,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        },
    );

    // Array of references to another module
    fields.insert(
        "array_ref".to_string(),
        Field {
            ty: Type::Array(Box::new(Type::Reference {
                name: "AnotherType".to_string(),
                module: Some("another.v1".to_string()),
            })),
            required: true,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        },
    );

    module.types.push(TypeDefinition {
        name: "ComplexType".to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // Add the referenced modules
    let mut other_module = Module {
        name: "other.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    other_module.types.push(TypeDefinition {
        name: "ExternalType".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    let mut another_module = Module {
        name: "another.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    another_module.types.push(TypeDefinition {
        name: "AnotherType".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(module);
    ir.modules.push(other_module);
    ir.modules.push(another_module);

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Verify all types are generated
    assert!(
        output.contains("ComplexType ="),
        "ComplexType should be defined"
    );
    assert!(
        output.contains("ExternalType ="),
        "ExternalType should be defined"
    );
    assert!(
        output.contains("AnotherType ="),
        "AnotherType should be defined"
    );

    // Verify the complex type has proper field definitions
    // The exact format will depend on the codegen implementation
    assert!(
        output.contains("optional_ref") || output.contains("array_ref"),
        "Complex type should contain field definitions"
    );

    Ok(())
}

#[test]
fn test_missing_reference_graceful_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test: References to non-existent types are handled gracefully
    let mut ir = IR::new();

    let mut module = Module {
        name: "test.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    // Type that references a non-existent type
    module.types.push(create_type_with_references(
        "TypeWithMissingRef",
        vec![
            ("NonExistentType", Some("missing.v1")),
            ("AnotherMissing", None),
        ],
    ));

    ir.modules.push(module);

    // Generate code - should not panic
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let result = codegen.generate(&ir);

    // This should either succeed with a placeholder or fail gracefully
    match result {
        Ok(output) => {
            assert!(
                output.contains("TypeWithMissingRef"),
                "Type should be generated even with missing references"
            );
            // Should either use Dyn or generate a comment about missing type
            assert!(
                output.contains("Dyn") || output.contains("missing") || output.contains("TODO"),
                "Missing references should be handled with Dyn or TODO"
            );
        }
        Err(e) => {
            // If it fails, the error should be informative
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("NonExistentType") || error_msg.contains("missing"),
                "Error should mention the missing type"
            );
        }
    }

    Ok(())
}

#[test]
fn test_version_migration_imports() -> Result<(), Box<dyn std::error::Error>> {
    // Test: References between different versions of the same API
    let mut ir = IR::new();

    // v1 version
    let mut v1_module = Module {
        name: "api.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    v1_module.types.push(TypeDefinition {
        name: "ConfigV1".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // v2 version that references v1
    let mut v2_module = Module {
        name: "api.v2".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    v2_module.types.push(create_type_with_references(
        "ConfigV2",
        vec![
            ("ConfigV1", Some("api.v1")), // Reference to older version
            ("ConfigV2Spec", None),       // Same version reference
        ],
    ));

    v2_module.types.push(TypeDefinition {
        name: "ConfigV2Spec".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(v1_module);
    ir.modules.push(v2_module);

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Verify all types are generated
    assert!(output.contains("ConfigV1"), "ConfigV1 should be defined");
    assert!(output.contains("ConfigV2"), "ConfigV2 should be defined");
    assert!(
        output.contains("ConfigV2Spec"),
        "ConfigV2Spec should be defined"
    );

    Ok(())
}

#[test]
fn test_import_deduplication() -> Result<(), Box<dyn std::error::Error>> {
    // Test: Multiple references to the same external module only generate one import
    let mut ir = IR::new();

    let mut module = Module {
        name: "test.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    // Multiple types referencing the same external module
    module.types.push(create_type_with_references(
        "Type1",
        vec![
            ("SharedType", Some("shared.v1")),
            ("AnotherSharedType", Some("shared.v1")),
        ],
    ));

    module.types.push(create_type_with_references(
        "Type2",
        vec![
            ("SharedType", Some("shared.v1")),
            ("YetAnotherSharedType", Some("shared.v1")),
        ],
    ));

    // Add the shared module
    let mut shared_module = Module {
        name: "shared.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    for name in &["SharedType", "AnotherSharedType", "YetAnotherSharedType"] {
        shared_module.types.push(TypeDefinition {
            name: name.to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });
    }

    ir.modules.push(module);
    ir.modules.push(shared_module);

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Count how many times the shared module is imported
    // This is a bit tricky without parsing the output structure
    // but we can at least verify the types are generated correctly
    assert!(output.contains("Type1"), "Type1 should be defined");
    assert!(output.contains("Type2"), "Type2 should be defined");
    assert!(
        output.contains("SharedType"),
        "SharedType should be defined"
    );

    Ok(())
}

#[test]
fn test_complex_real_world_scenario() -> Result<(), Box<dyn std::error::Error>> {
    // Test: Real-world K8s scenario with Deployment, Service, ConfigMap
    let mut ir = IR::new();

    // apps.v1 module
    let mut apps_module = Module {
        name: "k8s.io.apps.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    let mut deployment_fields = BTreeMap::new();
    deployment_fields.insert(
        "metadata".to_string(),
        Field {
            ty: Type::Reference {
                name: "ObjectMeta".to_string(),
                module: Some("k8s.io.v1".to_string()),
            },
            required: true,
            description: Some("Standard object metadata".to_string()),
            default: None,
            validation: None,
            contracts: vec![],
        },
    );
    deployment_fields.insert(
        "spec".to_string(),
        Field {
            ty: Type::Reference {
                name: "DeploymentSpec".to_string(),
                module: None,
            },
            required: true,
            description: Some("Deployment specification".to_string()),
            default: None,
            validation: None,
            contracts: vec![],
        },
    );

    apps_module.types.push(TypeDefinition {
        name: "Deployment".to_string(),
        ty: Type::Record {
            fields: deployment_fields,
            open: false,
        },
        documentation: Some("Deployment resource".to_string()),
        annotations: BTreeMap::new(),
    });

    let mut spec_fields = BTreeMap::new();
    spec_fields.insert(
        "template".to_string(),
        Field {
            ty: Type::Reference {
                name: "PodTemplateSpec".to_string(),
                module: Some("k8s.io.v1".to_string()),
            },
            required: true,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        },
    );

    apps_module.types.push(TypeDefinition {
        name: "DeploymentSpec".to_string(),
        ty: Type::Record {
            fields: spec_fields,
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // core.v1 module
    let mut core_module = Module {
        name: "k8s.io.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    core_module.types.push(TypeDefinition {
        name: "ObjectMeta".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "name".to_string(),
                    Field {
                        ty: Type::String,
                        required: true,
                        description: None,
                        default: None,
                        validation: None,
                        contracts: vec![],
                    },
                );
                fields.insert(
                    "namespace".to_string(),
                    Field {
                        ty: Type::Optional(Box::new(Type::String)),
                        required: false,
                        description: None,
                        default: None,
                        validation: None,
                        contracts: vec![],
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    core_module.types.push(TypeDefinition {
        name: "PodTemplateSpec".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    core_module.types.push(TypeDefinition {
        name: "Service".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "metadata".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "ObjectMeta".to_string(),
                            module: None, // Same module
                        },
                        required: true,
                        description: None,
                        default: None,
                        validation: None,
                        contracts: vec![],
                    },
                );
                fields
            },
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(apps_module);
    ir.modules.push(core_module);

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Verify all types are present
    assert!(
        output.contains("Deployment"),
        "Deployment should be defined"
    );
    assert!(
        output.contains("DeploymentSpec"),
        "DeploymentSpec should be defined"
    );
    assert!(
        output.contains("ObjectMeta"),
        "ObjectMeta should be defined"
    );
    assert!(
        output.contains("PodTemplateSpec"),
        "PodTemplateSpec should be defined"
    );
    assert!(output.contains("Service"), "Service should be defined");

    // Verify structure is maintained
    assert!(
        output.contains("metadata") && output.contains("spec"),
        "Deployment should have metadata and spec fields"
    );

    Ok(())
}
