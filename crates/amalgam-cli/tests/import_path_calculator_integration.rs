//! Integration test to verify all walkers use ImportPathCalculator correctly

use amalgam_core::{ImportPathCalculator, ModuleRegistry};
use amalgam_parser::walkers::{
    crd::{CRDInput, CRDVersion, CRDWalker},
    SchemaWalker,
};
use serde_json::json;
use std::sync::Arc;

/// Helper to create a test CRD with cross-version imports
fn create_test_crd_with_imports() -> CRDInput {
    CRDInput {
        group: "example.io".to_string(),
        versions: vec![
            CRDVersion {
                name: "v1".to_string(),
                schema: json!({
                    "openAPIV3Schema": {
                        "type": "object",
                        "properties": {
                            "spec": {
                                "type": "object",
                                "properties": {
                                    "metadata": {
                                        "$ref": "#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"
                                    },
                                    "containers": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/definitions/io.k8s.api.core.v1.Container"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }),
            },
            CRDVersion {
                name: "v1beta1".to_string(),
                schema: json!({
                    "openAPIV3Schema": {
                        "type": "object",
                        "properties": {
                            "spec": {
                                "type": "object",
                                "properties": {
                                    "metadata": {
                                        "$ref": "#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"
                                    }
                                }
                            }
                        }
                    }
                }),
            },
        ],
    }
}

#[test]
fn test_import_path_calculator_walker_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Create a CRD with cross-version imports
    let crd = create_test_crd_with_imports();

    // Process through CRD walker pipeline
    let walker = CRDWalker::new("example.io");
    let ir = walker.walk(crd)?;

    // Track that we found the expected imports
    let mut found_ncl_extension = false;
    let mut _found_cross_package = false;
    let mut found_proper_depth = true;

    // Debug what we got
    println!("Generated IR has {} modules", ir.modules.len());
    for module in &ir.modules {
        println!(
            "Module: {} with {} imports",
            module.name,
            module.imports.len()
        );

        for import in &module.imports {
            println!("  Import: {}", import.path);

            // All imports should end with .ncl
            assert!(
                import.path.ends_with(".ncl"),
                "Import path should end with .ncl: {}",
                import.path
            );
            found_ncl_extension = true;

            // Check depth of relative imports
            if import.path.starts_with("../") {
                let depth = import.path.matches("../").count();

                // Cross-package imports should have exactly 2 levels
                if import.path.contains("k8s_io") {
                    assert_eq!(
                        depth, 2,
                        "Cross-package import should have depth 2: {}",
                        import.path
                    );
                    _found_cross_package = true;
                } else {
                    // Cross-version within same package now has depth 2 for consolidated modules
                    assert_eq!(
                        depth, 2,
                        "Cross-version import should have depth 2: {}",
                        import.path
                    );
                }

                if depth > 2 {
                    found_proper_depth = false;
                }
            }
        }
    }

    // Ensure we actually tested something
    assert!(
        found_ncl_extension,
        "Should have found imports with .ncl extension"
    );
    assert!(found_proper_depth, "All imports should have proper depth");
    Ok(())
}

#[test]
fn test_import_calculator_direct_usage() -> Result<(), Box<dyn std::error::Error>> {
    let calc = ImportPathCalculator::new(Arc::new(ModuleRegistry::new()));

    // Test same package, same version - k8s.io types use consolidated modules
    let path = calc.calculate("k8s.io", "v1", "k8s.io", "v1", "Pod");
    assert_eq!(path, "../core/v1/mod.ncl");

    // Test same package, different version - now uses consolidated modules
    let path = calc.calculate("k8s.io", "v1beta1", "k8s.io", "v1", "ObjectMeta");
    assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");

    // Test different packages - now uses consolidated modules
    let path = calc.calculate("example.io", "v1", "k8s.io", "v1", "ObjectMeta");
    assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");

    // Test with crossplane - now uses consolidated modules
    let path = calc.calculate(
        "apiextensions.crossplane.io",
        "v1",
        "k8s.io",
        "v1",
        "ObjectMeta",
    );
    // CrossPlane packages now reference k8s.io consolidated modules
    assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");
    Ok(())
}

#[test]
fn test_walker_import_generation_consistency() -> Result<(), Box<dyn std::error::Error>> {
    // This test verifies that different walkers generate consistent import paths
    // when using the ImportPathCalculator

    let calc = ImportPathCalculator::new(Arc::new(ModuleRegistry::new()));

    // Simulate what each walker should generate - now using consolidated modules
    let test_cases = vec![
        // From package, from version, to package, to version, type, expected path
        (
            "k8s.io",
            "v1alpha3",
            "k8s.io",
            "v1alpha3",
            "CELDeviceSelector",
            "../core/v1alpha3/mod.ncl", // k8s.io types use consolidated modules
        ),
        (
            "k8s.io",
            "v1beta1",
            "k8s.io",
            "v1",
            "objectmeta",
            "../../apimachinery.pkg.apis/meta/v1/mod.ncl", // ObjectMeta is in apimachinery
        ),
        (
            "k8s.io",
            "v1",
            "k8s.io",
            "v0",
            "rawextension",
            "../../v0/mod.ncl", // RawExtension is in v0
        ),
        (
            "crossplane.io",
            "v1",
            "k8s.io",
            "v1",
            "pod",
            "../core/v1/mod.ncl", // Pod is in api/core/v1
        ),
    ];

    for (from_pkg, from_ver, to_pkg, to_ver, type_name, expected) in test_cases {
        let actual = calc.calculate(from_pkg, from_ver, to_pkg, to_ver, type_name);
        assert_eq!(
            actual, expected,
            "Import path mismatch for {} {} -> {} {} {}",
            from_pkg, from_ver, to_pkg, to_ver, type_name
        );
    }
    Ok(())
}
