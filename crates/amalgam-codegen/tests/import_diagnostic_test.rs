/// Import resolution diagnostic tool
/// Captures detailed information about how imports are being resolved
use amalgam_codegen::nickel::NickelCodegen;
use amalgam_codegen::Codegen;
use amalgam_core::ir::{Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use amalgam_core::ModuleRegistry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct ImportDiagnostic {
    test_name: String,
    input_modules: Vec<ModuleSummary>,
    generated_output: String,
    expected_patterns: Vec<String>,
    found_patterns: Vec<String>,
    missing_patterns: Vec<String>,
    unexpected_patterns: Vec<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModuleSummary {
    name: String,
    types: Vec<String>,
    references: Vec<ReferenceSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReferenceSummary {
    from_type: String,
    to_type: String,
    to_module: Option<String>,
}

fn diagnose_import_resolution(
    test_name: &str,
    ir: &IR,
    expected_patterns: Vec<&str>,
) -> ImportDiagnostic {
    // Capture input structure
    let input_modules: Vec<ModuleSummary> = ir
        .modules
        .iter()
        .map(|module| {
            let mut references = Vec::new();

            for type_def in &module.types {
                extract_references(&type_def.name, &type_def.ty, &mut references);
            }

            ModuleSummary {
                name: module.name.clone(),
                types: module.types.iter().map(|t| t.name.clone()).collect(),
                references,
            }
        })
        .collect();

    // Generate code
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let result = codegen.generate(ir);

    let (generated_output, error) = match result {
        Ok(output) => (output, None),
        Err(e) => (String::new(), Some(e.to_string())),
    };

    // Check patterns
    let expected_patterns: Vec<String> = expected_patterns.iter().map(|s| s.to_string()).collect();
    let mut found_patterns = Vec::new();
    let mut missing_patterns = Vec::new();

    for pattern in &expected_patterns {
        if generated_output.contains(pattern) {
            found_patterns.push(pattern.clone());
        } else {
            missing_patterns.push(pattern.clone());
        }
    }

    // Check for unexpected patterns (common bugs)
    let mut unexpected_patterns = Vec::new();

    // Check for lowercase type names (bug indicator)
    for module in &input_modules {
        for type_name in &module.types {
            let lowercase = type_name.to_lowercase();
            if type_name != &lowercase && generated_output.contains(&lowercase) {
                unexpected_patterns.push(format!("Unexpected lowercase: {}", lowercase));
            }
        }
    }

    // Check for missing import statements when there are cross-module refs
    let has_cross_module_refs = input_modules
        .iter()
        .any(|m| m.references.iter().any(|r| r.to_module.is_some()));

    if has_cross_module_refs
        && !generated_output.contains("import")
        && !generated_output.contains("Module.")
    {
        unexpected_patterns
            .push("Missing import statements for cross-module references".to_string());
    }

    ImportDiagnostic {
        test_name: test_name.to_string(),
        input_modules,
        generated_output,
        expected_patterns,
        found_patterns,
        missing_patterns,
        unexpected_patterns,
        error,
    }
}

fn extract_references(from_type: &str, ty: &Type, references: &mut Vec<ReferenceSummary>) {
    match ty {
        Type::Reference { name, module } => {
            references.push(ReferenceSummary {
                from_type: from_type.to_string(),
                to_type: name.clone(),
                to_module: module.clone(),
            });
        }
        Type::Optional(inner) => extract_references(from_type, inner, references),
        Type::Array(inner) => extract_references(from_type, inner, references),
        Type::Record { fields, .. } => {
            for field in fields.values() {
                extract_references(from_type, &field.ty, references);
            }
        }
        Type::Map { key, value } => {
            extract_references(from_type, key, references);
            extract_references(from_type, value, references);
        }
        Type::Union { types, .. } => {
            for t in types {
                extract_references(from_type, t, references);
            }
        }
        _ => {}
    }
}

#[test]
fn diagnose_all_import_scenarios() {
    let mut diagnostics = Vec::new();

    // Test 1: Same module references
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
            "spec".to_string(),
            Field {
                ty: Type::Reference {
                    name: "PodSpec".to_string(),
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
            name: "Pod".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        module.types.push(TypeDefinition {
            name: "PodSpec".to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        let diag = diagnose_import_resolution(
            "same_module_references",
            &ir,
            vec!["Pod =", "PodSpec =", "spec"],
        );
        diagnostics.push(diag);
    }

    // Test 2: Cross-module references
    {
        let mut ir = IR::new();

        // Module 1
        let mut module1 = Module {
            name: "apps.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "metadata".to_string(),
            Field {
                ty: Type::Reference {
                    name: "ObjectMeta".to_string(),
                    module: Some("core.v1".to_string()),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module1.types.push(TypeDefinition {
            name: "Deployment".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        // Module 2
        let mut module2 = Module {
            name: "core.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        module2.types.push(TypeDefinition {
            name: "ObjectMeta".to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module1);
        ir.modules.push(module2);

        let diag = diagnose_import_resolution(
            "cross_module_references",
            &ir,
            vec!["Deployment =", "ObjectMeta =", "metadata", "import"],
        );
        diagnostics.push(diag);
    }

    // Test 3: Missing reference target
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
            "missing".to_string(),
            Field {
                ty: Type::Reference {
                    name: "NonExistent".to_string(),
                    module: Some("missing.v1".to_string()),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module.types.push(TypeDefinition {
            name: "TypeWithMissing".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        let diag =
            diagnose_import_resolution("missing_reference", &ir, vec!["TypeWithMissing", "Dyn"]);
        diagnostics.push(diag);
    }

    // Test 4: Circular references
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

        let mut module2 = Module {
            name: "b.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        let mut fields = BTreeMap::new();
        fields.insert(
            "a_ref".to_string(),
            Field {
                ty: Type::Reference {
                    name: "TypeA".to_string(),
                    module: Some("a.v1".to_string()),
                },
                required: true,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );

        module2.types.push(TypeDefinition {
            name: "TypeB".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module1);
        ir.modules.push(module2);

        let diag = diagnose_import_resolution("circular_references", &ir, vec!["TypeA", "TypeB"]);
        diagnostics.push(diag);
    }

    // Test 5: Nested type references (Optional/Array with refs)
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
            "optional_ref".to_string(),
            Field {
                ty: Type::Optional(Box::new(Type::Reference {
                    name: "External".to_string(),
                    module: Some("other.v1".to_string()),
                })),
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
        fields.insert(
            "array_ref".to_string(),
            Field {
                ty: Type::Array(Box::new(Type::Reference {
                    name: "Another".to_string(),
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
        let mut other = Module {
            name: "other.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        other.types.push(TypeDefinition {
            name: "External".to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        let mut another = Module {
            name: "another.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        another.types.push(TypeDefinition {
            name: "Another".to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);
        ir.modules.push(other);
        ir.modules.push(another);

        let diag = diagnose_import_resolution(
            "nested_references",
            &ir,
            vec![
                "ComplexType",
                "External",
                "Another",
                "optional_ref",
                "array_ref",
            ],
        );
        diagnostics.push(diag);
    }

    // Print diagnostic report
    println!("\n=== IMPORT RESOLUTION DIAGNOSTIC REPORT ===\n");

    for diag in &diagnostics {
        println!("Test: {}", diag.test_name);
        println!("  Modules:");
        for module in &diag.input_modules {
            println!("    - {} with types: {:?}", module.name, module.types);
            for ref_sum in &module.references {
                println!(
                    "      {} -> {} (module: {:?})",
                    ref_sum.from_type, ref_sum.to_type, ref_sum.to_module
                );
            }
        }

        if let Some(error) = &diag.error {
            println!("  ERROR: {}", error);
        }

        if !diag.missing_patterns.is_empty() {
            println!("  Missing patterns: {:?}", diag.missing_patterns);
        }

        if !diag.unexpected_patterns.is_empty() {
            println!("  Unexpected patterns: {:?}", diag.unexpected_patterns);
        }

        println!("  Generated output preview:");
        let preview: String = diag
            .generated_output
            .lines()
            .take(15)
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", preview);

        println!("  ---");
    }

    // Save full diagnostics to file
    let json = serde_json::to_string_pretty(&diagnostics).unwrap();
    std::fs::write("import_diagnostics.json", json).unwrap();
    println!("\nFull diagnostics saved to import_diagnostics.json");

    // Summary
    let total_tests = diagnostics.len();
    let failed_tests = diagnostics
        .iter()
        .filter(|d| {
            !d.missing_patterns.is_empty() || !d.unexpected_patterns.is_empty() || d.error.is_some()
        })
        .count();

    println!("\n=== SUMMARY ===");
    println!("Total scenarios: {}", total_tests);
    println!("Failed scenarios: {}", failed_tests);

    if failed_tests > 0 {
        println!("\nKey issues found:");
        let mut issues = std::collections::HashSet::new();
        for diag in &diagnostics {
            for pattern in &diag.unexpected_patterns {
                issues.insert(pattern.clone());
            }
        }
        for issue in issues {
            println!("  - {}", issue);
        }
    }
}
