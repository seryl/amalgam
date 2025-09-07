use amalgam_codegen::nickel::NickelCodegen;
use amalgam_core::ir::{Metadata, Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use amalgam_core::ModuleRegistry;
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn test_import_tracking_same_module_references() -> Result<(), Box<dyn std::error::Error>> {
    // Create an IR with two types where one references the other
    let ir = IR {
        modules: vec![Module {
            name: "k8s.io.v1".to_string(),
            metadata: Metadata::default(),
            imports: vec![],
            types: vec![
                TypeDefinition {
                    name: "Lifecycle".to_string(),
                    ty: Type::Record {
                        fields: vec![(
                            "postStart".to_string(),
                            Field {
                                ty: Type::Reference {
                                    name: "LifecycleHandler".to_string(),
                                    module: None, // Same module reference
                                },
                                required: false,
                                default: None,
                                description: Some("PostStart handler".to_string()),
                            },
                        )]
                        .into_iter()
                        .collect(),
                        open: false,
                    },
                    documentation: Some("Lifecycle type".to_string()),
                    annotations: BTreeMap::new(),
                },
                TypeDefinition {
                    name: "LifecycleHandler".to_string(),
                    ty: Type::Record {
                        fields: vec![(
                            "exec".to_string(),
                            Field {
                                ty: Type::String,
                                required: false,
                                default: None,
                                description: Some("Exec command".to_string()),
                            },
                        )]
                        .into_iter()
                        .collect(),
                        open: false,
                    },
                    documentation: Some("LifecycleHandler type".to_string()),
                    annotations: BTreeMap::new(),
                },
            ],
            constants: vec![],
        }],
    };

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));

    // Use the new method
    let (output, import_map) = codegen
        .generate_with_import_tracking(&ir)
        ?;

    println!("=== Generated Output ===");
    println!("{}", output);

    println!("\n=== Import Map ===");
    println!(
        "Lifecycle imports: {:?}",
        import_map.get_imports_for("Lifecycle")
    );
    println!(
        "LifecycleHandler imports: {:?}",
        import_map.get_imports_for("LifecycleHandler")
    );

    // Check that Lifecycle has an import for LifecycleHandler
    let lifecycle_imports = import_map.get_imports_for("Lifecycle");
    assert!(
        !lifecycle_imports.is_empty(),
        "Lifecycle should have imports for LifecycleHandler"
    );

    // The import should be for same-version import (./lifecyclehandler.ncl)
    let import_str = &lifecycle_imports[0];
    assert!(
        import_str.contains("./lifecyclehandler.ncl"),
        "Import should be for same-version: {}",
        import_str
    );
    Ok(())
}
