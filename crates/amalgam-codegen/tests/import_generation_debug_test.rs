//! Test to diagnose import generation issues using debug data structures

use amalgam_codegen::nickel::NickelCodegen;
use amalgam_codegen::Codegen;
use amalgam_core::ir::{Metadata, Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use amalgam_core::ModuleRegistry;
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn test_debug_import_generation() -> Result<(), Box<dyn std::error::Error>> {
    // Create a minimal IR that reproduces the issue:
    // CSIPersistentVolumeSource references SecretReference
    let ir = IR {
        modules: vec![Module {
            name: "k8s.io.v1".to_string(),
            metadata: Metadata::default(),
            imports: vec![],
            types: vec![
                TypeDefinition {
                    name: "CSIPersistentVolumeSource".to_string(),
                    ty: Type::Record {
                        fields: vec![(
                            "controllerExpandSecretRef".to_string(),
                            Field {
                                ty: Type::Reference {
                                    name: "SecretReference".to_string(),
                                    module: None,
                                },
                                required: false,
                                default: None,
                                validation: None,
                                contracts: Vec::new(),
                                description: Some("Reference to secret".to_string()),
                            },
                        )]
                        .into_iter()
                        .collect(),
                        open: false,
                    },
                    documentation: Some("CSI volume source".to_string()),
                    annotations: BTreeMap::new(),
                },
                TypeDefinition {
                    name: "SecretReference".to_string(),
                    ty: Type::Record {
                        fields: vec![(
                            "name".to_string(),
                            Field {
                                ty: Type::String,
                                required: false,
                                default: None,
                                validation: None,
                                contracts: Vec::new(),
                                description: Some("Name of the secret".to_string()),
                            },
                        )]
                        .into_iter()
                        .collect(),
                        open: false,
                    },
                    documentation: Some("Reference to a secret".to_string()),
                    annotations: BTreeMap::new(),
                },
            ],
            constants: vec![],
        }],
    };

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let result = codegen.generate(&ir)?;

    // Print debug information
    println!("\n=== Import Generation Debug Info ===\n");

    println!(
        "Symbol Table Entries: {}",
        codegen.debug_info.symbol_table_entries.len()
    );
    for (type_name, (module, group, version)) in &codegen.debug_info.symbol_table_entries {
        println!(
            "  {} -> module: {}, group: {}, version: {}",
            type_name, module, group, version
        );
    }

    println!(
        "\nReferences Found: {}",
        codegen.debug_info.references_found.len()
    );
    for (from_module, referenced_type, resolved_to) in &codegen.debug_info.references_found {
        println!(
            "  In module '{}': references '{}' -> resolved to: {:?}",
            from_module, referenced_type, resolved_to
        );
    }

    println!(
        "\nDependencies Identified: {}",
        codegen.debug_info.dependencies_identified.len()
    );
    for (from_module, to_type, reason) in &codegen.debug_info.dependencies_identified {
        println!(
            "  Module '{}' depends on '{}' (reason: {})",
            from_module, to_type, reason
        );
    }

    println!(
        "\nImports Generated: {}",
        codegen.debug_info.imports_generated.len()
    );
    for (in_module, import_stmt) in &codegen.debug_info.imports_generated {
        println!("  In module '{}': {}", in_module, import_stmt);
    }

    println!(
        "\nMissing Types: {}",
        codegen.debug_info.missing_types.len()
    );
    for (module, type_name) in &codegen.debug_info.missing_types {
        println!(
            "  In module '{}': type '{}' not found in symbol table",
            module, type_name
        );
    }

    println!("\n=== Generated Output ===\n{}", result);

    // Assertions
    assert!(
        codegen
            .debug_info
            .references_found
            .iter()
            .any(|(_, typ, _)| typ == "SecretReference"),
        "Should have found SecretReference reference"
    );

    // Check if imports were generated
    assert!(
        !codegen.debug_info.imports_generated.is_empty()
            || codegen.debug_info.dependencies_identified.is_empty(),
        "If dependencies were identified, imports should be generated (or no deps needed)"
    );
    Ok(())
}
