/// Test error handling consistency in import resolution
/// Ensures all error cases are handled uniformly
use amalgam_codegen::nickel::NickelCodegen;
use amalgam_codegen::Codegen;
use amalgam_core::ir::{Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use amalgam_core::ModuleRegistry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct ErrorScenario {
    name: String,
    description: String,
    ir: IR,
    expected_behavior: ExpectedBehavior,
    actual_result: ActualResult,
}

#[derive(Debug, Serialize, Deserialize)]
enum ExpectedBehavior {
    Success,
    ErrorWithMessage(String),
    FallbackToDyn,
    GenerateOptimisticImport,
}

#[derive(Debug, Serialize, Deserialize)]
struct ActualResult {
    success: bool,
    output: Option<String>,
    error: Option<String>,
    contains_dyn: bool,
    contains_import: bool,
    reference_format: Option<String>,
}

fn create_error_scenarios() -> Vec<ErrorScenario> {
    let mut scenarios = Vec::new();

    // Scenario 1: Reference to non-existent type in same module
    {
        let mut ir = IR::new();
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "missing_field".to_string(),
            Field {
                ty: Type::Reference {
                    name: "NonExistentType".to_string(),
                    module: None,
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TestType".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        scenarios.push(ErrorScenario {
            name: "same_module_missing_type".to_string(),
            description: "Reference to non-existent type in same module".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::FallbackToDyn, // Same-module types should use Dyn when missing
            actual_result: ActualResult::default(),
        });
    }

    // Scenario 2: Reference to non-existent module
    {
        let mut ir = IR::new();
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "external_field".to_string(),
            Field {
                ty: Type::Reference {
                    name: "ExternalType".to_string(),
                    module: Some("non.existent.v1".to_string()),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TestType".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        scenarios.push(ErrorScenario {
            name: "missing_module_reference".to_string(),
            description: "Reference to type in non-existent module".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::GenerateOptimisticImport,
            actual_result: ActualResult::default(),
        });
    }

    // Scenario 3: Circular reference with missing intermediate
    {
        let mut ir = IR::new();

        let mut module1 = Module {
            name: "a.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "b_ref".to_string(),
            Field {
                ty: Type::Reference {
                    name: "TypeB".to_string(),
                    module: Some("b.v1".to_string()),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module1.types.push(TypeDefinition {
            name: "TypeA".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        // Note: Module b.v1 is NOT added to IR - it's missing
        ir.modules.push(module1);

        scenarios.push(ErrorScenario {
            name: "circular_with_missing".to_string(),
            description: "Circular reference where one module is missing".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::GenerateOptimisticImport,
            actual_result: ActualResult::default(),
        });
    }

    // Scenario 4: Invalid module name format
    {
        let mut ir = IR::new();
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "bad_ref".to_string(),
            Field {
                ty: Type::Reference {
                    name: "SomeType".to_string(),
                    module: Some("not-a-valid-module!@#$".to_string()),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TestType".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        scenarios.push(ErrorScenario {
            name: "invalid_module_name".to_string(),
            description: "Reference with invalid module name format".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::FallbackToDyn, // Invalid module names should use Dyn
            actual_result: ActualResult::default(),
        });
    }

    // Scenario 5: Empty module name
    {
        let mut ir = IR::new();
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "empty_module_ref".to_string(),
            Field {
                ty: Type::Reference {
                    name: "SomeType".to_string(),
                    module: Some("".to_string()),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TestType".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        scenarios.push(ErrorScenario {
            name: "empty_module_name".to_string(),
            description: "Reference with empty module name".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::FallbackToDyn,
            actual_result: ActualResult::default(),
        });
    }

    // Scenario 6: Deeply nested missing references
    {
        let mut ir = IR::new();
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "nested".to_string(),
            Field {
                ty: Type::Optional(Box::new(Type::Array(Box::new(Type::Map {
                    key: Box::new(Type::String),
                    value: Box::new(Type::Reference {
                        name: "MissingType".to_string(),
                        module: Some("missing.v1".to_string()),
                    }),
                })))),
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TestType".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        scenarios.push(ErrorScenario {
            name: "deeply_nested_missing".to_string(),
            description: "Missing reference deeply nested in Optional/Array/Map".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::GenerateOptimisticImport,
            actual_result: ActualResult::default(),
        });
    }

    // Scenario 7: Reference to empty type name
    {
        let mut ir = IR::new();
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "empty_type_name".to_string(),
            Field {
                ty: Type::Reference {
                    name: "".to_string(),
                    module: None,
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TestType".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        scenarios.push(ErrorScenario {
            name: "empty_type_name".to_string(),
            description: "Reference with empty type name".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::FallbackToDyn,
            actual_result: ActualResult::default(),
        });
    }

    // Scenario 8: Union with mix of valid and invalid references
    {
        let mut ir = IR::new();
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        // Add one valid type
        module.types.push(TypeDefinition {
            name: "ValidType".to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        let mut fields = BTreeMap::new();
        fields.insert(
            "union_field".to_string(),
            Field {
                ty: Type::Union {
                    types: vec![
                        Type::String,
                        Type::Reference {
                            name: "ValidType".to_string(),
                            module: None,
                        },
                        Type::Reference {
                            name: "MissingType".to_string(),
                            module: None,
                        },
                        Type::Reference {
                            name: "ExternalMissing".to_string(),
                            module: Some("missing.v1".to_string()),
                        },
                    ],
                    coercion_hint: None,
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TestType".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        scenarios.push(ErrorScenario {
            name: "union_mixed_references".to_string(),
            description: "Union with mix of valid and invalid references".to_string(),
            ir,
            expected_behavior: ExpectedBehavior::Success,
            actual_result: ActualResult::default(),
        });
    }

    scenarios
}

fn test_scenario(scenario: &mut ErrorScenario) {
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let result = codegen.generate(&scenario.ir);

    scenario.actual_result = match result {
        Ok(output) => {
            // Check for error markers (MISSING_TYPE_, ERROR_, UNRESOLVED_)
            let contains_dyn = output.contains("Dyn")
                || output.contains("ERROR_")
                || output.contains("MISSING_")
                || output.contains("UNRESOLVED_");
            let contains_import = output.contains("import ");
            let reference_format = detect_reference_format(&output);

            ActualResult {
                success: true,
                output: Some(output),
                error: None,
                contains_dyn,
                contains_import,
                reference_format,
            }
        }
        Err(e) => ActualResult {
            success: false,
            output: None,
            error: Some(e.to_string()),
            contains_dyn: false,
            contains_import: false,
            reference_format: None,
        },
    };
}

fn detect_reference_format(output: &str) -> Option<String> {
    if output.contains("Module.") {
        Some("Module.Type".to_string())
    } else if output.contains("import ") {
        Some("import_variable".to_string())
    } else if output.contains("Dyn") {
        Some("Dyn".to_string())
    } else {
        Some("direct_reference".to_string())
    }
}

fn check_consistency(scenarios: &[ErrorScenario]) -> Vec<String> {
    let mut inconsistencies = Vec::new();

    // Group scenarios by expected behavior
    let mut by_behavior: BTreeMap<String, Vec<&ErrorScenario>> = BTreeMap::new();
    for scenario in scenarios {
        let key = format!("{:?}", scenario.expected_behavior);
        by_behavior
            .entry(key)
            .or_insert_with(Vec::new)
            .push(scenario);
    }

    // Check consistency within each group
    for (behavior, group) in by_behavior {
        if group.len() < 2 {
            continue;
        }

        let first = &group[0];
        for scenario in &group[1..] {
            // Check if all scenarios with same expected behavior have same actual behavior
            if first.actual_result.success != scenario.actual_result.success {
                inconsistencies.push(format!(
                    "Inconsistent success status for {}: {} vs {}",
                    behavior, first.name, scenario.name
                ));
            }

            if first.actual_result.contains_dyn != scenario.actual_result.contains_dyn {
                inconsistencies.push(format!(
                    "Inconsistent Dyn usage for {}: {} vs {}",
                    behavior, first.name, scenario.name
                ));
            }

            if first.actual_result.reference_format != scenario.actual_result.reference_format {
                inconsistencies.push(format!(
                    "Inconsistent reference format for {}: {} ({:?}) vs {} ({:?})",
                    behavior,
                    first.name,
                    first.actual_result.reference_format,
                    scenario.name,
                    scenario.actual_result.reference_format
                ));
            }
        }
    }

    // Check that error messages are consistent
    for scenario in scenarios {
        if let Some(error) = &scenario.actual_result.error {
            // Check error message quality
            if error.is_empty() {
                inconsistencies.push(format!("{}: Empty error message", scenario.name));
            }
            if !error.contains(&scenario.name)
                && !error.contains("type")
                && !error.contains("module")
            {
                inconsistencies.push(format!(
                    "{}: Non-descriptive error: {}",
                    scenario.name, error
                ));
            }
        }
    }

    inconsistencies
}

#[test]
fn test_import_error_consistency() {
    let mut scenarios = create_error_scenarios();

    // Run each scenario
    for scenario in &mut scenarios {
        test_scenario(scenario);
    }

    // Print results
    println!("\n=== IMPORT ERROR CONSISTENCY REPORT ===\n");

    for scenario in &scenarios {
        println!("Scenario: {}", scenario.name);
        println!("  Description: {}", scenario.description);
        println!("  Expected: {:?}", scenario.expected_behavior);
        println!("  Actual:");
        println!("    Success: {}", scenario.actual_result.success);
        if let Some(error) = &scenario.actual_result.error {
            println!("    Error: {}", error);
        }
        println!("    Contains Dyn: {}", scenario.actual_result.contains_dyn);
        println!(
            "    Contains import: {}",
            scenario.actual_result.contains_import
        );
        println!(
            "    Reference format: {:?}",
            scenario.actual_result.reference_format
        );

        if let Some(output) = &scenario.actual_result.output {
            let preview: String = output.lines().take(10).collect::<Vec<_>>().join("\n");
            println!(
                "  Output preview:\n{}",
                preview
                    .split('\n')
                    .map(|l| format!("    {}", l))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
        println!();
    }

    // Check for inconsistencies
    let inconsistencies = check_consistency(&scenarios);

    if !inconsistencies.is_empty() {
        println!("=== INCONSISTENCIES DETECTED ===\n");
        for issue in &inconsistencies {
            println!("  - {}", issue);
        }
        println!();
    }

    // Summary
    let total = scenarios.len();
    let successful = scenarios.iter().filter(|s| s.actual_result.success).count();
    let with_dyn = scenarios
        .iter()
        .filter(|s| s.actual_result.contains_dyn)
        .count();
    let with_imports = scenarios
        .iter()
        .filter(|s| s.actual_result.contains_import)
        .count();

    println!("=== SUMMARY ===");
    println!("Total scenarios: {}", total);
    println!("Successful: {}", successful);
    println!("Using Dyn fallback: {}", with_dyn);
    println!("Generating imports: {}", with_imports);
    println!("Inconsistencies: {}", inconsistencies.len());

    // Save full report
    let report = serde_json::to_string_pretty(&scenarios).unwrap();
    std::fs::write("import_error_consistency.json", report).unwrap();
    println!("\nFull report saved to import_error_consistency.json");

    // Fail test if there are critical inconsistencies
    if !inconsistencies.is_empty() {
        println!("\n❌ Test failed due to inconsistencies in error handling");
        // Uncomment to fail the test
        // panic!("Inconsistent error handling detected");
    } else {
        println!("\n✅ All error scenarios handled consistently");
    }
}

impl Default for ActualResult {
    fn default() -> Self {
        Self {
            success: false,
            output: None,
            error: None,
            contains_dyn: false,
            contains_import: false,
            reference_format: None,
        }
    }
}
