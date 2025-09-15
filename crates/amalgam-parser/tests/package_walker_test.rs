//! Direct tests for PackageWalkerAdapter functionality

use amalgam_core::{
    ir::TypeDefinition,
    types::{Field, Type},
};
use amalgam_parser::package_walker::PackageWalkerAdapter;
use std::collections::{BTreeMap, HashMap};

/// Create test type definitions for testing
fn create_test_type_definitions() -> HashMap<String, TypeDefinition> {
    let mut types = HashMap::new();

    // Simple type
    types.insert(
        "pod".to_string(),
        TypeDefinition {
            name: "Pod".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "metadata".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "ObjectMeta".to_string(),
                                module: Some("k8s.io.v1".to_string()),
                            },
                            required: true,
                            description: Some("Standard object metadata".to_string()),
                            default: None,
                        },
                    );
                    fields.insert(
                        "spec".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "PodSpec".to_string(),
                                module: Some("k8s.io.v1".to_string()),
                            },
                            required: true,
                            description: Some("Pod specification".to_string()),
                            default: None,
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: Some("Pod represents a pod in Kubernetes".to_string()),
            annotations: Default::default(),
        },
    );

    // Type with internal reference
    types.insert(
        "podspec".to_string(),
        TypeDefinition {
            name: "PodSpec".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "containers".to_string(),
                        Field {
                            ty: Type::Array(Box::new(Type::Reference {
                                name: "Container".to_string(),
                                module: Some("k8s.io.v1".to_string()),
                            })),
                            required: true,
                            description: Some("List of containers".to_string()),
                            default: None,
                        },
                    );
                    fields.insert(
                        "restartPolicy".to_string(),
                        Field {
                            ty: Type::String,
                            required: false,
                            description: Some("Restart policy for containers".to_string()),
                            default: Some(serde_json::json!("Always")),
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: Some("PodSpec is the specification of a pod".to_string()),
            annotations: Default::default(),
        },
    );

    // Simple type without references
    types.insert(
        "container".to_string(),
        TypeDefinition {
            name: "Container".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "name".to_string(),
                        Field {
                            ty: Type::String,
                            required: true,
                            description: Some("Container name".to_string()),
                            default: None,
                        },
                    );
                    fields.insert(
                        "image".to_string(),
                        Field {
                            ty: Type::String,
                            required: true,
                            description: Some("Container image".to_string()),
                            default: None,
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: Some("Container represents a container in a pod".to_string()),
            annotations: Default::default(),
        },
    );

    // Metadata type
    types.insert(
        "objectmeta".to_string(),
        TypeDefinition {
            name: "ObjectMeta".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "name".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::String)),
                            required: false,
                            description: Some("Name of the object".to_string()),
                            default: None,
                        },
                    );
                    fields.insert(
                        "namespace".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::String)),
                            required: false,
                            description: Some("Namespace of the object".to_string()),
                            default: None,
                        },
                    );
                    fields.insert(
                        "labels".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::Map {
                                key: Box::new(Type::String),
                                value: Box::new(Type::String),
                            })),
                            required: false,
                            description: Some("Labels for the object".to_string()),
                            default: None,
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: Some("ObjectMeta is metadata for all objects".to_string()),
            annotations: Default::default(),
        },
    );

    types
}

#[test]
fn test_package_walker_build_registry() -> Result<(), Box<dyn std::error::Error>> {
    let types = create_test_type_definitions();

    // Test registry building
    let registry = PackageWalkerAdapter::build_registry(&types, "k8s.io", "v1")?;

    // Verify all types were added to registry
    assert_eq!(registry.types.len(), types.len());

    // Verify FQN format
    assert!(registry.types.contains_key("k8s.io.v1.pod"));
    assert!(registry.types.contains_key("k8s.io.v1.podspec"));
    assert!(registry.types.contains_key("k8s.io.v1.container"));
    assert!(registry.types.contains_key("k8s.io.v1.objectmeta"));

    // Verify type content is preserved
    let pod = registry
        .types
        .get("k8s.io.v1.pod")
        .ok_or("Type not found")?;
    assert_eq!(pod.name, "Pod");
    assert!(pod.documentation.is_some());
    Ok(())
}

#[test]
fn test_package_walker_build_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let types = create_test_type_definitions();
    let registry = PackageWalkerAdapter::build_registry(&types, "k8s.io", "v1")?;

    // Test dependency extraction
    let deps = PackageWalkerAdapter::build_dependencies(&registry);

    // Pod should depend on ObjectMeta and PodSpec
    let pod_deps = deps.get_dependencies("k8s.io.v1.pod");
    assert!(!pod_deps.is_empty());
    assert!(pod_deps.contains(&"k8s.io.v1.objectmeta".to_string()));
    assert!(pod_deps.contains(&"k8s.io.v1.podspec".to_string()));

    // PodSpec should depend on Container
    let podspec_deps = deps.get_dependencies("k8s.io.v1.podspec");
    assert!(podspec_deps.contains(&"k8s.io.v1.container".to_string()));

    // Container should have no dependencies
    let container_deps = deps.get_dependencies("k8s.io.v1.container");
    assert!(container_deps.is_empty());

    // ObjectMeta should have no dependencies (only primitive types)
    let meta_deps = deps.get_dependencies("k8s.io.v1.objectmeta");
    assert!(meta_deps.is_empty());
    Ok(())
}

#[test]
fn test_package_walker_generate_ir() -> Result<(), Box<dyn std::error::Error>> {
    let types = create_test_type_definitions();
    let registry = PackageWalkerAdapter::build_registry(&types, "k8s.io", "v1")?;
    let deps = PackageWalkerAdapter::build_dependencies(&registry);

    // Test IR generation
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "k8s.io", "v1")?;

    // Should have modules for each type
    assert_eq!(ir.modules.len(), types.len());

    // Check module names follow FQN pattern
    for module in &ir.modules {
        assert!(module.name.starts_with("k8s.io.v1."));

        // Module should have exactly one type
        assert_eq!(module.types.len(), 1);

        // Check for imports
        if module.name.contains("pod") && !module.name.contains("podspec") {
            // Pod module should have no imports (same package references)
            // since ObjectMeta and PodSpec are in the same package
            assert!(
                module.imports.is_empty()
                    || module.imports.iter().all(|i| i.path.starts_with("./"))
            );
        }
    }
    Ok(())
}

#[test]
fn test_cross_module_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let mut types = HashMap::new();

    // Type in v1 that references v1beta1
    types.insert(
        "deployment".to_string(),
        TypeDefinition {
            name: "Deployment".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "oldSpec".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "DeploymentSpec".to_string(),
                                module: Some("k8s.io.v1beta1".to_string()),
                            },
                            required: true,
                            description: Some("Legacy spec".to_string()),
                            default: None,
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: None,
            annotations: Default::default(),
        },
    );

    let registry = PackageWalkerAdapter::build_registry(&types, "k8s.io", "v1")?;
    let deps = PackageWalkerAdapter::build_dependencies(&registry);
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "k8s.io", "v1")?;

    // Find the deployment module
    let deployment_module = ir
        .modules
        .iter()
        .find(|m| m.name.contains("deployment"))
        .ok_or("Module not found")?;

    // Should have cross-version import
    assert!(!deployment_module.imports.is_empty());

    // Import should use ImportPathCalculator logic
    for import in &deployment_module.imports {
        if import.path.contains("deploymentspec") {
            // Should be ../v1beta1/deploymentspec.ncl
            assert!(import.path.contains("../"));
            assert!(import.path.contains("v1beta1"));
            assert!(import.path.ends_with(".ncl"));
        }
    }
    Ok(())
}

#[test]
fn test_empty_types() -> Result<(), Box<dyn std::error::Error>> {
    let types = HashMap::new();

    let registry = PackageWalkerAdapter::build_registry(&types, "test.io", "v1")?;
    assert!(registry.types.is_empty());

    let deps = PackageWalkerAdapter::build_dependencies(&registry);
    assert!(deps.get_all_dependencies().is_empty());

    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "test.io", "v1")?;
    assert!(ir.modules.is_empty());
    Ok(())
}

#[test]
fn test_complex_type_references() -> Result<(), Box<dyn std::error::Error>> {
    let mut types = HashMap::new();

    // Type with various reference types
    types.insert(
        "complex".to_string(),
        TypeDefinition {
            name: "Complex".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();

                    // Direct reference
                    fields.insert(
                        "direct".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "Simple".to_string(),
                                module: Some("test.io.v1".to_string()),
                            },
                            required: true,
                            description: None,
                            default: None,
                        },
                    );

                    // Optional reference
                    fields.insert(
                        "optional".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::Reference {
                                name: "Another".to_string(),
                                module: Some("test.io.v1".to_string()),
                            })),
                            required: false,
                            description: None,
                            default: None,
                        },
                    );

                    // Array of references
                    fields.insert(
                        "array".to_string(),
                        Field {
                            ty: Type::Array(Box::new(Type::Reference {
                                name: "Item".to_string(),
                                module: Some("test.io.v1".to_string()),
                            })),
                            required: true,
                            description: None,
                            default: None,
                        },
                    );

                    // Map with reference values
                    fields.insert(
                        "map".to_string(),
                        Field {
                            ty: Type::Map {
                                key: Box::new(Type::String),
                                value: Box::new(Type::Reference {
                                    name: "Value".to_string(),
                                    module: Some("test.io.v1".to_string()),
                                }),
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
            documentation: None,
            annotations: Default::default(),
        },
    );

    let registry = PackageWalkerAdapter::build_registry(&types, "test.io", "v1")?;
    let deps = PackageWalkerAdapter::build_dependencies(&registry);

    // Should extract all reference types
    let complex_deps = deps.get_dependencies("test.io.v1.complex");
    assert_eq!(complex_deps.len(), 4);
    assert!(complex_deps.contains(&"test.io.v1.simple".to_string()));
    assert!(complex_deps.contains(&"test.io.v1.another".to_string()));
    assert!(complex_deps.contains(&"test.io.v1.item".to_string()));
    assert!(complex_deps.contains(&"test.io.v1.value".to_string()));
    Ok(())
}
