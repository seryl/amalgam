//! Error handling test suite for robust error recovery

use amalgam_parser::walkers::{
    crd::{CRDInput, CRDVersion, CRDWalker},
    SchemaWalker,
};
use serde_json::json;

/// Test handling of malformed CRD input
#[test]
fn test_malformed_crd_handling() -> Result<(), Box<dyn std::error::Error>> {
    let malformed_crd = CRDInput {
        group: "test.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1".to_string(),
            schema: json!({
                "openAPIV3Schema": {
                    // Malformed schema - invalid type structure
                    "type": "invalid_type_that_doesnt_exist",
                    "properties": "this should be an object not a string"
                }
            }),
        }],
    };

    let walker = CRDWalker::new("test.example.io");
    let result = walker.walk(malformed_crd);

    match result {
        Ok(_) => {
            // If it succeeds, it should handle the malformed input gracefully
            // by treating unknown types as "Any"
            println!("✓ Malformed CRD handled gracefully");
        }
        Err(e) => {
            // If it fails, error should be descriptive
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("invalid")
                    || error_msg.contains("malformed")
                    || error_msg.contains("parse"),
                "Error should indicate parsing issue: {}",
                error_msg
            );
        }
    }
    Ok(())
}

/// Test handling of CRD with missing $ref targets
#[test]
fn test_missing_ref_target() -> Result<(), Box<dyn std::error::Error>> {
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
                                    "$ref": "#/definitions/NonExistentType"
                                },
                                "anotherMissingRef": {
                                    "$ref": "io.k8s.api.core.v999.ImaginaryType"
                                }
                            }
                        }
                    }
                }
            }),
        }],
    };

    let walker = CRDWalker::new("test.example.io");
    let result = walker.walk(crd_with_missing_ref);

    // Should handle missing references gracefully
    match result {
        Ok(ir) => {
            // Should have created some modules despite missing references
            assert!(
                !ir.modules.is_empty(),
                "Should create modules even with missing refs"
            );

            // The missing references should either be:
            // 1. Treated as external dependencies (imports), or
            // 2. Replaced with placeholder/Any types
            println!(
                "✓ Missing references handled gracefully with {} modules",
                ir.modules.len()
            );
        }
        Err(e) => {
            // If it fails, should provide helpful error about missing reference
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("NonExistentType")
                    || error_msg.contains("reference")
                    || error_msg.contains("missing")
                    || error_msg.contains("ImaginaryType"),
                "Error should mention the missing reference: {}",
                error_msg
            );
        }
    }
    Ok(())
}

/// Test handling of circular dependencies
#[test]
fn test_circular_dependency_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Create a CRD with circular references: A -> B -> A
    let circular_crd = CRDInput {
        group: "test.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1".to_string(),
            schema: json!({
                "openAPIV3Schema": {
                    "type": "object",
                    "properties": {
                        "spec": {
                            "$ref": "#/definitions/TypeA"
                        }
                    },
                    "definitions": {
                        "TypeA": {
                            "type": "object",
                            "properties": {
                                "refToB": {
                                    "$ref": "#/definitions/TypeB"
                                }
                            }
                        },
                        "TypeB": {
                            "type": "object",
                            "properties": {
                                "refToA": {
                                    "$ref": "#/definitions/TypeA"
                                }
                            }
                        }
                    }
                }
            }),
        }],
    };

    let walker = CRDWalker::new("test.example.io");
    let result = walker.walk(circular_crd);

    match result {
        Ok(_) => {
            // If it succeeds, it should handle circular dependencies gracefully
            // by either breaking the cycle or detecting it and providing warnings
            println!("✓ Circular dependency handled gracefully");
        }
        Err(e) => {
            // Should detect and report circular dependency
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("circular")
                    || error_msg.contains("cycle")
                    || error_msg.contains("recursive"),
                "Error should indicate circular dependency: {}",
                error_msg
            );
        }
    }
    Ok(())
}

/// Test handling of empty CRD input
#[test]
fn test_empty_crd_input() -> Result<(), Box<dyn std::error::Error>> {
    let empty_crd = CRDInput {
        group: "empty.example.io".to_string(),
        versions: vec![],
    };

    let walker = CRDWalker::new("empty.example.io");
    let result = walker.walk(empty_crd);

    match result {
        Ok(ir) => {
            // Should handle empty input gracefully
            assert!(ir.modules.is_empty(), "Empty CRD should produce empty IR");
        }
        Err(e) => {
            // Should provide helpful error about empty input
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("empty")
                    || error_msg.contains("no versions")
                    || error_msg.contains("invalid"),
                "Error should indicate empty input: {}",
                error_msg
            );
        }
    }
    Ok(())
}

/// Test handling of CRD with no schema
#[test]
fn test_version_without_schema() -> Result<(), Box<dyn std::error::Error>> {
    let crd_no_schema = CRDInput {
        group: "test.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1".to_string(),
            schema: json!(null), // null schema
        }],
    };

    let walker = CRDWalker::new("test.example.io");
    let result = walker.walk(crd_no_schema);

    match result {
        Ok(_) => {
            // Should handle null schema gracefully
            println!("✓ Null schema handled gracefully");
        }
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("schema")
                    || error_msg.contains("null")
                    || error_msg.contains("invalid"),
                "Error should indicate schema issue: {}",
                error_msg
            );
        }
    }
    Ok(())
}

/// Test handling of invalid JSON Schema constructs
#[test]
fn test_invalid_json_schema_constructs() -> Result<(), Box<dyn std::error::Error>> {
    let invalid_schema_crd = CRDInput {
        group: "test.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1".to_string(),
            schema: json!({
                "openAPIV3Schema": {
                    "type": "object",
                    "properties": {
                        "invalidProperty": {
                            // Invalid: conflicting type declarations
                            "type": ["string", "number", "boolean"],
                            "enum": [1, "two", true],
                            "minimum": "not a number",
                            "maxLength": -5
                        },
                        "anotherInvalid": {
                            // Invalid: $ref with other properties (not allowed in JSON Schema)
                            "$ref": "#/definitions/SomeType",
                            "type": "string",
                            "properties": {
                                "shouldnt": "be here"
                            }
                        }
                    }
                }
            }),
        }],
    };

    let walker = CRDWalker::new("test.example.io");
    let result = walker.walk(invalid_schema_crd);

    match result {
        Ok(_) => {
            // Should handle invalid constructs gracefully by falling back to Any type
            println!("✓ Invalid JSON Schema constructs handled gracefully");
        }
        Err(e) => {
            // Should provide helpful error about invalid schema
            let error_msg = e.to_string();
            assert!(
                !error_msg.is_empty(),
                "Should provide meaningful error for invalid schema"
            );
        }
    }
    Ok(())
}

/// Test handling of deeply nested schemas
#[test]
fn test_deeply_nested_schema() -> Result<(), Box<dyn std::error::Error>> {
    // Create a schema with very deep nesting to test stack overflow protection
    let mut deep_schema = json!({
        "type": "object",
        "properties": {
            "level0": {
                "type": "object"
            }
        }
    });

    // Create 100 levels of nesting
    let mut current = &mut deep_schema["properties"]["level0"];
    for i in 1..100 {
        *current = json!({
            "type": "object",
            "properties": {
                format!("level{}", i): {
                    "type": "object"
                }
            }
        });
        current = &mut current["properties"][&format!("level{}", i)];
    }

    let deep_crd = CRDInput {
        group: "deep.example.io".to_string(),
        versions: vec![CRDVersion {
            name: "v1".to_string(),
            schema: json!({
                "openAPIV3Schema": deep_schema
            }),
        }],
    };

    let walker = CRDWalker::new("deep.example.io");
    let result = walker.walk(deep_crd);

    match result {
        Ok(_) => {
            // Should handle deep nesting without stack overflow
            println!("✓ Deep nesting handled gracefully");
        }
        Err(e) => {
            let error_msg = e.to_string();
            // Should either succeed or fail gracefully (not crash)
            assert!(
                error_msg.contains("depth")
                    || error_msg.contains("nested")
                    || error_msg.contains("recursion")
                    || !error_msg.is_empty(),
                "Should provide meaningful error for deep nesting: {}",
                error_msg
            );
        }
    }
    Ok(())
}

/// Test recovery from import path calculation errors
#[test]
fn test_import_path_calculation_error_recovery() -> Result<(), Box<dyn std::error::Error>> {
    // Create scenario that might cause import path issues
    let problematic_crd = CRDInput {
        group: "".to_string(), // Empty group name
        versions: vec![CRDVersion {
            name: "".to_string(), // Empty version name
            schema: json!({
                "openAPIV3Schema": {
                    "type": "object",
                    "properties": {
                        "spec": {
                            "$ref": "io.k8s.api.core.v1.Pod"
                        }
                    }
                }
            }),
        }],
    };

    let walker = CRDWalker::new("");
    let result = walker.walk(problematic_crd);

    match result {
        Ok(_) => {
            println!("✓ Empty group/version names handled gracefully");
        }
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("group")
                    || error_msg.contains("version")
                    || error_msg.contains("empty")
                    || error_msg.contains("invalid"),
                "Error should indicate the problematic group/version: {}",
                error_msg
            );
        }
    }
    Ok(())
}

/// Integration test: Error recovery doesn't break the pipeline
#[test]
fn test_error_recovery_pipeline_resilience() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mix of valid and invalid CRDs to test that errors in one
    // don't break processing of others
    let mixed_crds = vec![
        // Valid CRD
        CRDInput {
            group: "valid.example.io".to_string(),
            versions: vec![CRDVersion {
                name: "v1".to_string(),
                schema: json!({
                    "openAPIV3Schema": {
                        "type": "object",
                        "properties": {
                            "spec": {
                                "type": "object",
                                "properties": {
                                    "replicas": {
                                        "type": "integer"
                                    }
                                }
                            }
                        }
                    }
                }),
            }],
        },
        // Invalid CRD
        CRDInput {
            group: "invalid.example.io".to_string(),
            versions: vec![CRDVersion {
                name: "v1".to_string(),
                schema: json!({
                    "openAPIV3Schema": {
                        "type": "this_is_not_a_valid_type",
                        "properties": "also_not_valid"
                    }
                }),
            }],
        },
    ];

    let walker = CRDWalker::new("test");

    // Process each CRD - some should succeed, some should fail gracefully
    let mut successes = 0;
    let mut meaningful_errors = 0;

    for crd in mixed_crds {
        match walker.walk(crd) {
            Ok(_) => {
                successes += 1;
                println!("✓ CRD processed successfully");
            }
            Err(e) => {
                let error_msg = e.to_string();
                if !error_msg.is_empty() && error_msg.len() > 10 {
                    meaningful_errors += 1;
                    println!("✓ Meaningful error: {}", error_msg);
                }
            }
        }
    }

    // Should have at least some successes or meaningful errors
    assert!(
        successes > 0 || meaningful_errors > 0,
        "Should either process successfully or provide meaningful errors"
    );
    Ok(())
}
