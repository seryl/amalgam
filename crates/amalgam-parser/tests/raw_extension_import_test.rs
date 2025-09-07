//! Test that RawExtension and other runtime types correctly import from v0

use amalgam_core::{
    ir::TypeDefinition,
    types::{Field, Type},
    ImportPathCalculator,
};
use amalgam_parser::package_walker::PackageWalkerAdapter;
use std::collections::{BTreeMap, HashMap};

/// Create a type that references RawExtension
fn create_type_with_raw_extension() -> HashMap<String, TypeDefinition> {
    let mut types = HashMap::new();

    // Create a v1 type that uses RawExtension
    types.insert(
        "customresource".to_string(),
        TypeDefinition {
            name: "CustomResource".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();

                    // Field that references RawExtension (runtime type)
                    fields.insert(
                        "extension".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::Reference {
                                name: "RawExtension".to_string(),
                                module: Some("io.k8s.apimachinery.pkg.runtime".to_string()),
                            })),
                            required: false,
                            description: Some("Raw extension data".to_string()),
                            default: None,
                        },
                    );

                    // Regular v1 reference for comparison
                    fields.insert(
                        "metadata".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "ObjectMeta".to_string(),
                                module: Some("io.k8s.apimachinery.pkg.apis.meta.v1".to_string()),
                            },
                            required: true,
                            description: Some("Standard metadata".to_string()),
                            default: None,
                        },
                    );

                    fields
                },
                open: false,
            },
            documentation: Some("Custom resource with raw extension".to_string()),
            annotations: Default::default(),
        },
    );

    types
}

#[test]
fn test_rawextension_v0_import() -> Result<(), Box<dyn std::error::Error>> {
    let types = create_type_with_raw_extension();

    // Process through package walker
    let registry = PackageWalkerAdapter::build_registry(&types, "example.io", "v1")
        ?;
    let deps = PackageWalkerAdapter::build_dependencies(&registry);
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "example.io", "v1")
        ?;

    // Find imports in the generated IR
    let mut found_raw_extension = false;
    let mut found_object_meta = false;

    for module in &ir.modules {
        for import in &module.imports {
            println!("Found import: {}", import.path);

            if import.path.contains("rawextension") {
                // RawExtension should import from v0
                assert!(
                    import.path.contains("/v0/"),
                    "RawExtension should import from v0, got: {}",
                    import.path
                );
                assert_eq!(
                    import.path, "../../k8s_io/v0/rawextension.ncl",
                    "RawExtension import path should be correct"
                );
                found_raw_extension = true;
            }

            if import.path.contains("objectmeta") {
                // ObjectMeta should import from v1
                assert!(
                    import.path.contains("/v1/"),
                    "ObjectMeta should import from v1, got: {}",
                    import.path
                );
                found_object_meta = true;
            }
        }
    }

    assert!(
        found_raw_extension,
        "Should have found RawExtension import from v0"
    );
    assert!(
        found_object_meta,
        "Should have found ObjectMeta import from v1"
    );
    Ok(())
}

#[test]
fn test_runtime_types_version_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test that runtime and pkg types are correctly identified as v0
    let test_cases = vec![
        ("io.k8s.apimachinery.pkg.runtime.RawExtension", "v0"),
        ("io.k8s.apimachinery.pkg.runtime.Unknown", "v0"),
        ("io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta", "v1"),
        ("io.k8s.api.core.v1.Pod", "v1"),
        ("io.k8s.api.core.v1beta1.Pod", "v1beta1"),
    ];

    for (fqn, expected_version) in test_cases {
        let version = extract_version_from_fqn(fqn);
        assert_eq!(
            version, expected_version,
            "Version extraction failed for {}",
            fqn
        );
    }
    Ok(())
}

/// Helper to extract version from FQN (mimics logic in package_walker.rs)
fn extract_version_from_fqn(fqn: &str) -> &str {
    if fqn.contains(".v1.") || fqn.contains(".meta.v1.") {
        "v1"
    } else if fqn.contains(".v1alpha1.") {
        "v1alpha1"
    } else if fqn.contains(".v1alpha3.") {
        "v1alpha3"
    } else if fqn.contains(".v1beta1.") {
        "v1beta1"
    } else if fqn.contains(".v2.") {
        "v2"
    } else if fqn.contains(".runtime.") || fqn.contains(".pkg.") {
        // Unversioned runtime types go in v0
        "v0"
    } else {
        "v1"
    }
}

#[test]
fn test_import_path_calculator_v0_imports() -> Result<(), Box<dyn std::error::Error>> {
    let calc = ImportPathCalculator::new_standalone();

    // Test v1 -> v0 import for RawExtension
    let path = calc.calculate("k8s.io", "v1", "k8s.io", "v0", "rawextension");
    assert_eq!(path, "../v0/rawextension.ncl");

    // Test from different package to v0
    let path = calc.calculate("example.io", "v1", "k8s.io", "v0", "rawextension");
    assert_eq!(path, "../../k8s_io/v0/rawextension.ncl");

    // Test v1beta1 -> v0
    let path = calc.calculate("k8s.io", "v1beta1", "k8s.io", "v0", "unknown");
    assert_eq!(path, "../v0/unknown.ncl");
    Ok(())
}

#[test]
fn test_multiple_runtime_type_references() -> Result<(), Box<dyn std::error::Error>> {
    let mut types = HashMap::new();

    // Create a type with multiple runtime references
    types.insert(
        "webhookconfig".to_string(),
        TypeDefinition {
            name: "WebhookConfig".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();

                    // Multiple runtime type references
                    fields.insert(
                        "raw".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "RawExtension".to_string(),
                                module: Some("io.k8s.apimachinery.pkg.runtime".to_string()),
                            },
                            required: false,
                            description: None,
                            default: None,
                        },
                    );

                    fields.insert(
                        "unknown".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::Reference {
                                name: "Unknown".to_string(),
                                module: Some("io.k8s.apimachinery.pkg.runtime".to_string()),
                            })),
                            required: false,
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

    // Generate IR
    let registry = PackageWalkerAdapter::build_registry(&types, "webhooks.io", "v1")
        ?;
    let deps = PackageWalkerAdapter::build_dependencies(&registry);
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "webhooks.io", "v1")
        ?;

    // All runtime imports should go to v0
    for module in &ir.modules {
        for import in &module.imports {
            if import.path.contains("rawextension") || import.path.contains("unknown") {
                assert!(
                    import.path.contains("/v0/"),
                    "Runtime type should import from v0: {}",
                    import.path
                );
            }
        }
    }
    Ok(())
}
