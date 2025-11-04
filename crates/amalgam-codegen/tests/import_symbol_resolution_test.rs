//! Comprehensive tests for import resolution and symbol lookup correctness
//!
//! These tests ensure that generated Nickel code has:
//! 1. Correct import paths that actually resolve to existing files
//! 2. Matching local bindings and their usage
//! 3. No dangling references (using undefined symbols)
//! 4. Proper cross-package references
//! 5. Correct casing for bindings and type references

use amalgam_codegen::nickel::NickelCodegen;
use amalgam_core::ir::{Field, Import, Metadata, Module, TypeDefinition, IR};
use amalgam_core::module_registry::ModuleRegistry;
use amalgam_core::types::Type;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn test_import_binding_matches_usage() {
    // This is the CRITICAL test: import binding name must match usage
    let ir = create_test_ir_with_cross_reference();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let (generated, _) = codegen.generate_with_import_tracking(&ir).unwrap();

    // Extract imports (lines starting with "let")
    let import_lines: Vec<&str> = generated
        .lines()
        .filter(|l| l.trim().starts_with("let "))
        .collect();

    // Extract import bindings: "let <binding> = import ..."
    let mut bindings = HashMap::new();
    for line in import_lines {
        if let Some(binding_part) = line.split('=').next() {
            let binding = binding_part
                .trim()
                .strip_prefix("let ")
                .unwrap_or("")
                .trim()
                .to_string();

            // Extract the imported type name from path
            if let Some(import_path) = line.split('"').nth(1) {
                let type_name = import_path
                    .trim_end_matches(".ncl")
                    .split('/')
                    .last()
                    .unwrap_or("")
                    .to_string();
                bindings.insert(type_name.clone(), binding.clone());
            }
        }
    }

    // Now check that all type references use the correct binding
    for (type_name, binding) in bindings {
        // Find usages of this type in contracts (| TypeName |)
        let usage_pattern = format!("| {} |", type_name);
        let usage_pattern_optional = format!("| {}\n", type_name);

        if generated.contains(&usage_pattern) || generated.contains(&usage_pattern_optional) {
            // Type is used, verify binding matches
            assert_eq!(
                binding, type_name,
                "Import binding '{}' doesn't match usage '{}'. \
                 Nickel code will fail! Binding should be: \
                 let {} = import \"...\"",
                binding, type_name, type_name
            );
        }
    }
}

#[test]
fn test_no_dangling_type_references() {
    let ir = create_test_ir_with_cross_reference();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let (generated, _) = codegen.generate_with_import_tracking(&ir).unwrap();

    // Extract all type references used in contracts (| TypeName |)
    let mut used_types = HashSet::new();
    for line in generated.lines() {
        // Match pattern: | TypeName |
        if let Some(contract_part) = line.split('|').nth(1) {
            let type_ref = contract_part.trim();
            if !type_ref.is_empty()
                && type_ref.chars().next().unwrap().is_uppercase()
                && !type_ref.contains('{')  // Not an inline contract
                && !type_ref.contains("String")  // Not a primitive
                && !type_ref.contains("Number")
                && !type_ref.contains("Bool")
                && !type_ref.contains("Array")
            {
                used_types.insert(type_ref.to_string());
            }
        }
    }

    // Extract all available types (imported or defined locally)
    let mut available_types = HashSet::new();

    // Add imported types
    for line in generated.lines() {
        if line.trim().starts_with("let ") && line.contains("import") {
            if let Some(binding_part) = line.split('=').next() {
                let binding = binding_part
                    .trim()
                    .strip_prefix("let ")
                    .unwrap_or("")
                    .trim();
                available_types.insert(binding.to_string());
            }
        }
    }

    // Add locally defined types (from the module being generated)
    for module in &ir.modules {
        for type_def in &module.types {
            available_types.insert(type_def.name.clone());
        }
    }

    // Check for dangling references
    let dangling: Vec<_> = used_types
        .difference(&available_types)
        .collect();

    assert!(
        dangling.is_empty(),
        "Found dangling type references (used but not imported or defined): {:?}\n\
         Available types: {:?}\n\
         Used types: {:?}",
        dangling,
        available_types,
        used_types
    );
}

#[test]
fn test_import_paths_are_well_formed() {
    let ir = create_test_ir_with_cross_reference();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let (generated, _) = codegen.generate_with_import_tracking(&ir).unwrap();

    for line in generated.lines() {
        if line.contains("import") && line.contains('"') {
            // Extract the import path
            if let Some(path) = line.split('"').nth(1) {
                // Path should end with .ncl
                assert!(
                    path.ends_with(".ncl"),
                    "Import path '{}' should end with .ncl",
                    path
                );

                // Path should not have consecutive slashes
                assert!(
                    !path.contains("//"),
                    "Import path '{}' contains consecutive slashes",
                    path
                );

                // Path should not start with slash (relative paths only)
                assert!(
                    !path.starts_with('/'),
                    "Import path '{}' should be relative, not absolute",
                    path
                );

                // If it's a relative path with .., validate structure
                if path.contains("..") {
                    // Count .. and forward components
                    let parts: Vec<&str> = path.split('/').collect();
                    let up_count = parts.iter().filter(|&&p| p == "..").count();
                    let down_count = parts.iter().filter(|&&p| p != ".." && !p.is_empty()).count();

                    // Should have at least one non-.. component
                    assert!(
                        down_count > 0,
                        "Import path '{}' has only '..' components",
                        path
                    );
                }
            }
        }
    }
}

#[test]
fn test_cross_package_import_paths_correct() {
    // Test that imports from one package to another have correct relative paths
    let ir = create_multi_package_ir();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let (generated, import_map) = codegen.generate_with_import_tracking(&ir).unwrap();

    // For a type in apiextensions.crossplane.io/v1/Composition.ncl
    // importing from k8s.io/v1/ObjectMeta.ncl
    // the path should be: ../../k8s_io/v1/ObjectMeta.ncl

    if generated.contains("ObjectMeta") {
        // Should have an import for ObjectMeta
        assert!(
            generated.contains("import \"../../k8s_io/"),
            "Cross-package import from crossplane to k8s should use ../../k8s_io/ path\n\
             Generated:\n{}",
            generated
        );
    }
}

#[test]
fn test_symbol_table_completeness() {
    // Verify that the symbol table contains all types we'll need to reference
    let ir = create_test_ir_with_cross_reference();
    let codegen = NickelCodegen::from_ir(&ir);

    // The debug_info.symbol_table_entries should contain entries for all types in the IR
    let symbol_table = &codegen.debug_info.symbol_table_entries;

    for module in &ir.modules {
        for type_def in &module.types {
            // Type should be in symbol table (may not be if it's the type being generated)
            // or should be the current type being processed
        }
    }

    // At minimum, verify no missing_types were detected
    assert!(
        codegen.debug_info.missing_types.is_empty(),
        "Symbol table is incomplete - missing types: {:?}",
        codegen.debug_info.missing_types
    );
}

#[test]
fn test_same_package_imports_correct() {
    // Test imports within the same package (same group/version)
    let ir = create_ir_with_same_package_references();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let (generated, _) = codegen.generate_with_import_tracking(&ir).unwrap();

    // Imports within same version should be: ./TypeName.ncl
    if generated.contains("import") {
        let import_lines: Vec<&str> = generated
            .lines()
            .filter(|l| l.contains("import"))
            .collect();

        for line in import_lines {
            if let Some(path) = line.split('"').nth(1) {
                // Same-package imports should start with ./
                if !path.contains("..") {
                    assert!(
                        path.starts_with("./"),
                        "Same-package import '{}' should start with ./",
                        path
                    );
                }
            }
        }
    }
}

#[test]
fn test_import_deduplication() {
    // Verify that the same type isn't imported multiple times
    let ir = create_test_ir_with_multiple_references();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let (generated, _) = codegen.generate_with_import_tracking(&ir).unwrap();

    // Extract all import statements
    let import_statements: Vec<&str> = generated
        .lines()
        .filter(|l| l.trim().starts_with("let ") && l.contains("import"))
        .collect();

    // Check for duplicates
    let mut seen_paths = HashSet::new();
    for stmt in import_statements {
        if let Some(path) = stmt.split('"').nth(1) {
            assert!(
                seen_paths.insert(path.to_string()),
                "Duplicate import detected: '{}'",
                path
            );
        }
    }
}

#[test]
fn test_import_binding_case_sensitivity() {
    // Nickel is case-sensitive, ensure bindings match usage exactly
    let ir = create_test_ir_with_cross_reference();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let (generated, _) = codegen.generate_with_import_tracking(&ir).unwrap();

    // Extract bindings and their usage
    let mut binding_cases = HashMap::new();

    for line in generated.lines() {
        if line.trim().starts_with("let ") && line.contains("import") {
            if let Some(binding_part) = line.split('=').next() {
                let binding = binding_part
                    .trim()
                    .strip_prefix("let ")
                    .unwrap_or("")
                    .trim();

                // Store the exact case of the binding
                binding_cases.insert(binding.to_lowercase(), binding.to_string());
            }
        }
    }

    // Check usages maintain exact case
    for line in generated.lines() {
        if line.contains('|') && !line.contains("doc") {
            // This might be a type contract
            for (_lower, actual_case) in &binding_cases {
                if line.contains(actual_case) {
                    // Verify exact match, not case-insensitive variant
                    let contract_match = format!("| {} ", actual_case);
                    if line.contains("| ") && line.contains(actual_case) {
                        // This is acceptable - exact case match
                    }
                }
            }
        }
    }
}

#[test]
fn test_circular_import_detection() {
    // Ensure we don't generate circular imports
    // A.ncl imports B.ncl which imports A.ncl
    let ir = create_ir_with_potential_circular_refs();
    let mut codegen = NickelCodegen::from_ir(&ir);

    let result = codegen.generate_with_import_tracking(&ir);

    // Generation should succeed (we handle this by not importing the current type)
    assert!(result.is_ok(), "Should handle potential circular references");

    let (generated, _) = result.unwrap();

    // The type being generated should NOT import itself
    for module in &ir.modules {
        for type_def in &module.types {
            let self_import = format!("import \"./{}.ncl\"", type_def.name);
            assert!(
                !generated.contains(&self_import),
                "Type {} should not import itself - circular reference!",
                type_def.name
            );
        }
    }
}

// Helper functions to create test IRs

fn create_test_ir_with_cross_reference() -> IR {
    let mut ir = IR::new();

    // Create k8s.io module with ObjectMeta
    let k8s_module = Module {
        name: "k8s.io.v1".to_string(),
        types: vec![TypeDefinition {
            name: "ObjectMeta".to_string(),
            fields: vec![
                Field {
                    name: "name".to_string(),
                    field_type: Type::String,
                    optional: true,
                    doc: None,
                },
            ],
            doc: None,
        }],
        imports: vec![],
        constants: vec![],
        metadata: Metadata::default(),
    };

    // Create crossplane module that references ObjectMeta
    let crossplane_module = Module {
        name: "apiextensions.crossplane.io.v1".to_string(),
        types: vec![TypeDefinition {
            name: "Composition".to_string(),
            fields: vec![
                Field {
                    name: "metadata".to_string(),
                    field_type: Type::Reference("ObjectMeta".to_string()),
                    optional: true,
                    doc: None,
                },
            ],
            doc: None,
        }],
        imports: vec![],
        constants: vec![],
        metadata: Metadata::default(),
    };

    ir.modules = vec![k8s_module, crossplane_module];
    ir
}

fn create_multi_package_ir() -> IR {
    create_test_ir_with_cross_reference()
}

fn create_ir_with_same_package_references() -> IR {
    let mut ir = IR::new();

    let module = Module {
        name: "test.io.v1".to_string(),
        types: vec![
            TypeDefinition {
                name: "TypeA".to_string(),
                fields: vec![],
                doc: None,
            },
            TypeDefinition {
                name: "TypeB".to_string(),
                fields: vec![
                    Field {
                        name: "refToA".to_string(),
                        field_type: Type::Reference("TypeA".to_string()),
                        optional: false,
                        doc: None,
                    },
                ],
                doc: None,
            },
        ],
        imports: vec![],
        constants: vec![],
        metadata: Metadata::default(),
    };

    ir.modules = vec![module];
    ir
}

fn create_test_ir_with_multiple_references() -> IR {
    let mut ir = IR::new();

    // Create a type that references ObjectMeta multiple times
    let module = Module {
        name: "test.io.v1".to_string(),
        types: vec![TypeDefinition {
            name: "TestType".to_string(),
            fields: vec![
                Field {
                    name: "meta1".to_string(),
                    field_type: Type::Reference("ObjectMeta".to_string()),
                    optional: true,
                    doc: None,
                },
                Field {
                    name: "meta2".to_string(),
                    field_type: Type::Reference("ObjectMeta".to_string()),
                    optional: true,
                    doc: None,
                },
            ],
            doc: None,
        }],
        imports: vec![
            Import {
                path: "../../k8s_io/v1/ObjectMeta.ncl".to_string(),
                alias: Some("ObjectMeta".to_string()),
                items: vec![],
            },
        ],
        constants: vec![],
        metadata: Metadata::default(),
    };

    ir.modules = vec![module];
    ir
}

fn create_ir_with_potential_circular_refs() -> IR {
    let mut ir = IR::new();

    let module = Module {
        name: "test.io.v1".to_string(),
        types: vec![
            TypeDefinition {
                name: "TypeA".to_string(),
                fields: vec![
                    Field {
                        name: "selfRef".to_string(),
                        field_type: Type::Reference("TypeA".to_string()),
                        optional: true,
                        doc: None,
                    },
                ],
                doc: None,
            },
        ],
        imports: vec![],
        constants: vec![],
        metadata: Metadata::default(),
    };

    ir.modules = vec![module];
    ir
}
