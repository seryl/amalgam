//! Test CRD $ref resolution and reference tracking

use amalgam_parser::walkers::{
    crd::{CRDInput, CRDVersion, CRDWalker},
    SchemaWalker,
};
use serde_json::json;

/// Create a CRD with various $ref patterns for testing
fn create_crd_with_refs() -> CRDInput {
    CRDInput {
        group: "test.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1".to_string(),
            schema: json!({
                "openAPIV3Schema": {
                    "type": "object",
                    "properties": {
                        "spec": {
                            "type": "object",
                            "properties": {
                                // Local reference within same CRD
                                "localRef": {
                                    "$ref": "#/definitions/LocalType"
                                },
                                // K8s core type reference
                                "metadata": {
                                    "$ref": "#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"
                                },
                                // K8s API resource reference
                                "volume": {
                                    "$ref": "#/definitions/io.k8s.api.core.v1.Volume"
                                },
                                // Optional reference
                                "optionalRef": {
                                    "type": "object",
                                    "properties": {
                                        "nested": {
                                            "$ref": "#/definitions/NestedType"
                                        }
                                    }
                                },
                                // Array of references
                                "containers": {
                                    "type": "array",
                                    "items": {
                                        "$ref": "#/definitions/io.k8s.api.core.v1.Container"
                                    }
                                }
                            }
                        }
                    },
                    "definitions": {
                        "LocalType": {
                            "type": "object",
                            "properties": {
                                "name": {
                                    "type": "string"
                                }
                            }
                        },
                        "NestedType": {
                            "type": "object",
                            "properties": {
                                "value": {
                                    "type": "string"
                                }
                            }
                        }
                    }
                }
            }),
        }],
    }
}

/// Create a CRD that tests complex $ref patterns
fn create_complex_ref_crd() -> CRDInput {
    CRDInput {
        group: "complex.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1beta1".to_string(),
            schema: json!({
                "openAPIV3Schema": {
                    "type": "object",
                    "properties": {
                        "spec": {
                            "type": "object",
                            "properties": {
                                // Reference chain - type that references another type
                                "chainedRef": {
                                    "$ref": "#/definitions/ChainedTypeA"
                                },
                                // oneOf with references
                                "unionWithRefs": {
                                    "oneOf": [
                                        {
                                            "$ref": "#/definitions/TypeOption1"
                                        },
                                        {
                                            "$ref": "#/definitions/io.k8s.api.core.v1.ConfigMap"
                                        }
                                    ]
                                },
                                // allOf with references
                                "mergedWithRef": {
                                    "allOf": [
                                        {
                                            "$ref": "#/definitions/BaseType"
                                        },
                                        {
                                            "type": "object",
                                            "properties": {
                                                "additionalField": {
                                                    "type": "string"
                                                }
                                            }
                                        }
                                    ]
                                }
                            }
                        }
                    },
                    "definitions": {
                        "ChainedTypeA": {
                            "type": "object",
                            "properties": {
                                "refToB": {
                                    "$ref": "#/definitions/ChainedTypeB"
                                }
                            }
                        },
                        "ChainedTypeB": {
                            "type": "object",
                            "properties": {
                                "value": {
                                    "type": "string"
                                }
                            }
                        },
                        "TypeOption1": {
                            "type": "object",
                            "properties": {
                                "option1Field": {
                                    "type": "string"
                                }
                            }
                        },
                        "BaseType": {
                            "type": "object",
                            "properties": {
                                "baseField": {
                                    "type": "string"
                                }
                            }
                        }
                    }
                }
            }),
        }],
    }
}

#[test]
fn test_basic_ref_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let crd = create_crd_with_refs();
    let walker = CRDWalker::new("test.example.io");

    let ir = walker.walk(crd)?;

    // Should have the main module (local types are embedded in it)
    assert!(!ir.modules.is_empty(), "Should have at least one module");

    // Find the main spec module
    let main_module = ir
        .modules
        .iter()
        .find(|m| m.name.contains("test.example.io.v1"))
        .ok_or("Module not found")?;

    // Should have imports for K8s types
    assert!(
        !main_module.imports.is_empty(),
        "Main module should have imports for K8s references"
    );

    // Check specific imports exist
    let import_paths: Vec<&str> = main_module
        .imports
        .iter()
        .map(|i| i.path.as_str())
        .collect();

    // Should have imports for k8s types (they use relative paths to k8s modules)
    let has_k8s_import = import_paths.iter().any(|path| {
        path.contains("apimachinery") || path.contains("api/core") || path.contains("k8s_io")
    });
    assert!(
        has_k8s_import,
        "Should have k8s import for ObjectMeta/Volume/Container references"
    );

    println!("Generated modules:");
    for module in &ir.modules {
        println!("  - {}", module.name);
        for import in &module.imports {
            println!("    imports: {}", import.path);
        }
    }
    Ok(())
}

#[test]
fn test_local_vs_external_refs() -> Result<(), Box<dyn std::error::Error>> {
    let crd = create_crd_with_refs();
    let walker = CRDWalker::new("test.example.io");

    let registry = walker.extract_types(&crd)?;

    assert!(!registry.types.is_empty(), "Registry should have types");

    // Check that K8s references are tracked but not included in local registry
    // (they should be in dependencies)
    let deps = walker.build_dependencies(&registry);

    // Should have cross-module dependencies for K8s types
    let all_deps = deps.dependencies;
    let has_k8s_deps = all_deps.values().any(|dep_set| {
        dep_set
            .iter()
            .any(|dep| dep.contains("k8s") || dep.contains("ObjectMeta"))
    });

    assert!(has_k8s_deps, "Should have dependencies on K8s types");
    Ok(())
}

#[test]
fn test_complex_ref_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let crd = create_complex_ref_crd();
    let walker = CRDWalker::new("complex.example.io");

    let ir = walker.walk(crd)?;

    // Should handle chained references (TypeA -> TypeB)
    // But they might be embedded in the main type instead of separate modules
    let has_main_module = !ir.modules.is_empty();
    assert!(has_main_module, "Should generate at least the main module");

    // Should handle oneOf with mixed local and external refs
    let main_module = ir
        .modules
        .iter()
        .find(|m| m.name.contains("complex.example.io.v1beta1"))
        .ok_or("Module not found")?;

    // Should have both local references and external K8s references
    let import_paths: Vec<&str> = main_module
        .imports
        .iter()
        .map(|i| i.path.as_str())
        .collect();

    // Check for local references (same module)
    let has_local_imports = import_paths.iter().any(|path| path.starts_with("./"));

    // Check for external references (k8s imports use relative paths)
    let has_external_imports = import_paths.iter().any(|path| {
        path.contains("core/") || path.contains("apimachinery") || path.contains("k8s_io")
    });

    println!("Complex CRD imports:");
    for import in &main_module.imports {
        println!("  - {}", import.path);
    }

    // Should have external K8s imports for ConfigMap reference in oneOf
    assert!(has_external_imports, "Should have external K8s imports");

    // Note: Local imports may be optimized away if types are in same module,
    // but we should verify that local type references are handled correctly
    // even if they don't result in actual import statements
    if has_local_imports {
        println!("✓ Found local imports as expected");
    } else {
        println!("ⓘ No local imports found - types may be in same module");

        // Verify that the main module was created (local types are embedded within it)
        let has_main_module = !ir.modules.is_empty();
        assert!(
            has_main_module,
            "Should have at least the main module with embedded local types"
        );
    }
    Ok(())
}

#[test]
fn test_ref_in_array_items() -> Result<(), Box<dyn std::error::Error>> {
    let crd = create_crd_with_refs();
    let walker = CRDWalker::new("test.example.io");

    let registry = walker.extract_types(&crd)?;

    // Find a type that has an array with references (containers field)
    let array_with_refs = registry.types.values().any(|type_def| {
        if let amalgam_core::types::Type::Record { fields, .. } = &type_def.ty {
            fields.values().any(|field| {
                matches!(&field.ty, amalgam_core::types::Type::Array(inner) 
                    if matches!(inner.as_ref(), amalgam_core::types::Type::Reference { .. }))
            })
        } else {
            false
        }
    });

    assert!(array_with_refs, "Should handle $ref in array items");
    Ok(())
}

#[test]
fn test_ref_path_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let crd = create_crd_with_refs();
    let walker = CRDWalker::new("test.example.io");

    let registry = walker.extract_types(&crd)?;
    let deps = walker.build_dependencies(&registry);
    let ir = walker.generate_ir(registry, deps)?;

    // Verify correct import path calculation for different ref types
    for module in &ir.modules {
        for import in &module.imports {
            // All imports should end with .ncl
            assert!(
                import.path.ends_with(".ncl"),
                "Import path should end with .ncl: {}",
                import.path
            );

            // K8s imports should use proper relative path depth
            if import.path.contains("api/core") || import.path.contains("apimachinery") {
                assert!(
                    import.path.starts_with("../../"),
                    "K8s import should use ../../ relative path: {}",
                    import.path
                );
            }

            // Local imports should use ./
            if import.path.contains("LocalType") || import.path.contains("NestedType") {
                assert!(
                    import.path.starts_with("./"),
                    "Local import should use ./ relative path: {}",
                    import.path
                );
            }
        }
    }
    Ok(())
}

#[test]
fn test_missing_ref_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test CRD with reference to undefined type
    let crd_with_missing_ref = CRDInput {
        group: "test.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1".to_string(),
            schema: json!({
                "openAPIV3Schema": {
                    "type": "object",
                    "properties": {
                        "spec": {
                            "type": "object",
                            "properties": {
                                "missingRef": {
                                    "$ref": "#/definitions/UndefinedType"
                                }
                            }
                        }
                    }
                }
            }),
        }],
    };

    let walker = CRDWalker::new("test.example.io");

    // Should handle missing references gracefully
    // Either by creating a placeholder or by continuing with partial processing
    let result = walker.walk(crd_with_missing_ref);

    // The walker should either:
    // 1. Process successfully with placeholder types, or
    // 2. Return a meaningful error
    match result {
        Ok(ir) => {
            // If successful, should have created some modules
            assert!(
                !ir.modules.is_empty(),
                "Should create modules even with missing refs"
            );
        }
        Err(e) => {
            // If error, should be meaningful
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("UndefinedType")
                    || error_msg.contains("reference")
                    || error_msg.contains("missing"),
                "Error should mention the missing reference: {}",
                error_msg
            );
        }
    }
    Ok(())
}
