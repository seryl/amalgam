use amalgam_codegen::nickel::NickelCodegen;
use amalgam_core::ir::{Metadata, Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use std::collections::BTreeMap;

#[test]
fn test_pipeline_debug() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple IR with Lifecycle referencing LifecycleHandler
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
                                validation: None,
                                contracts: Vec::new(),
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
                                validation: None,
                                contracts: Vec::new(),
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

    let mut codegen = NickelCodegen::from_ir(&ir);
    let (_output, import_map) = codegen.generate_with_import_tracking(&ir)?;

    // Output the pipeline debug
    println!("=== Pipeline Debug Summary ===");
    println!("{}", codegen.pipeline_debug.summary_string());

    // Get detailed reports
    println!("\n=== Lifecycle Report ===");
    println!("{}", codegen.pipeline_debug.type_report("Lifecycle"));

    println!("\n=== LifecycleHandler Report ===");
    println!("{}", codegen.pipeline_debug.type_report("LifecycleHandler"));

    // Check imports
    let lifecycle_imports = import_map.get_imports_for("Lifecycle");
    println!("\n=== TypeImportMap for Lifecycle ===");
    println!("{:?}", lifecycle_imports);

    // Check that Lifecycle has dependencies
    let lifecycle_deps = codegen.pipeline_debug.dependency_analysis.get("Lifecycle");
    assert!(
        lifecycle_deps.is_some(),
        "Lifecycle should have dependency analysis"
    );

    let deps = lifecycle_deps.ok_or("Lifecycle deps not found")?;
    assert!(
        !deps.dependencies_identified.is_empty(),
        "Lifecycle should have LifecycleHandler as dependency. Found: {:?}",
        deps
    );

    // Check that imports were generated
    let imports = codegen.pipeline_debug.import_generation.get("Lifecycle");
    assert!(
        imports.is_some(),
        "Lifecycle should have import generation record"
    );
    assert!(
        !imports
            .ok_or("Imports not found")?
            .import_statements
            .is_empty(),
        "Lifecycle should have import statements generated"
    );
    Ok(())
}
