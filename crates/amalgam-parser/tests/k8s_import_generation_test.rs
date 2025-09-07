//! Test that k8s type generation produces proper imports for cross-type references

use amalgam_core::ir::TypeDefinition;
use amalgam_core::types::{Field, Type};
use amalgam_parser::package_walker::PackageWalkerAdapter;
use std::collections::{BTreeMap, HashMap};

#[test]
fn test_k8s_lifecycle_imports() -> Result<(), Box<dyn std::error::Error>> {
    // Create types for k8s v1 version

    // Create LifecycleHandler type
    let lifecycle_handler = TypeDefinition {
        name: "LifecycleHandler".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "exec".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "ExecAction".to_string(),
                            module: None,
                        },
                        required: false,
                        description: Some("Exec specifies a command to execute".to_string()),
                        default: None,
                    },
                );
                fields.insert(
                    "httpGet".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "HTTPGetAction".to_string(),
                            module: None,
                        },
                        required: false,
                        description: Some("HTTPGet specifies an HTTP GET request".to_string()),
                        default: None,
                    },
                );
                fields
            },
            open: false,
        },
        documentation: Some("LifecycleHandler defines actions for container lifecycle".to_string()),
        annotations: Default::default(),
    };

    // Create Lifecycle type that references LifecycleHandler
    let lifecycle = TypeDefinition {
        name: "Lifecycle".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "postStart".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "LifecycleHandler".to_string(),
                            module: None,
                        },
                        required: false,
                        description: Some(
                            "PostStart is called after container creation".to_string(),
                        ),
                        default: None,
                    },
                );
                fields.insert(
                    "preStop".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "LifecycleHandler".to_string(),
                            module: None,
                        },
                        required: false,
                        description: Some(
                            "PreStop is called before container termination".to_string(),
                        ),
                        default: None,
                    },
                );
                fields
            },
            open: false,
        },
        documentation: Some("Lifecycle describes container lifecycle actions".to_string()),
        annotations: Default::default(),
    };

    // Create ExecAction type
    let exec_action = TypeDefinition {
        name: "ExecAction".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "command".to_string(),
                    Field {
                        ty: Type::Array(Box::new(Type::String)),
                        required: false,
                        description: Some("Command to execute".to_string()),
                        default: None,
                    },
                );
                fields
            },
            open: false,
        },
        documentation: Some("ExecAction describes a command to execute".to_string()),
        annotations: Default::default(),
    };

    // Create HTTPGetAction type
    let http_get_action = TypeDefinition {
        name: "HTTPGetAction".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "path".to_string(),
                    Field {
                        ty: Type::String,
                        required: false,
                        description: Some("Path to request".to_string()),
                        default: None,
                    },
                );
                fields.insert(
                    "port".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "IntOrString".to_string(),
                            module: Some("k8s.io.v0".to_string()),
                        },
                        required: true,
                        description: Some("Port to connect to".to_string()),
                        default: None,
                    },
                );
                fields
            },
            open: false,
        },
        documentation: Some("HTTPGetAction describes an HTTP GET request".to_string()),
        annotations: Default::default(),
    };

    // Add types to a hashmap
    let mut v1_types = HashMap::new();
    v1_types.insert("Lifecycle".to_string(), lifecycle);
    v1_types.insert("LifecycleHandler".to_string(), lifecycle_handler);
    v1_types.insert("ExecAction".to_string(), exec_action);
    v1_types.insert("HTTPGetAction".to_string(), http_get_action);

    // Build registry and dependencies using PackageWalkerAdapter
    let registry = PackageWalkerAdapter::build_registry(&v1_types, "k8s.io", "v1")
        ?;

    let deps = PackageWalkerAdapter::build_dependencies(&registry);

    // Generate IR
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "k8s.io", "v1")
        ?;

    // Verify we have a single module
    assert_eq!(ir.modules.len(), 1, "Should have exactly one module");
    let module = &ir.modules[0];
    assert_eq!(module.name, "k8s.io.v1", "Module should be named k8s.io.v1");

    // Verify all types are in the module
    assert_eq!(module.types.len(), 4, "Should have 4 types in module");

    // Now generate Nickel code and check for imports
    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let (output, import_map) = codegen
        .generate_with_import_tracking(&ir)
        ?;

    // Check that import map has entries
    let lifecycle_imports = import_map.get_imports_for("Lifecycle");
    println!("Lifecycle imports: {:?}", lifecycle_imports);
    println!("Full output:\n{}", output);
    assert!(
        !lifecycle_imports.is_empty(),
        "Lifecycle should have imports"
    );
    assert!(
        lifecycle_imports
            .iter()
            .any(|i| i.contains("LifecycleHandler")),
        "Lifecycle should import LifecycleHandler"
    );

    let handler_imports = import_map.get_imports_for("LifecycleHandler");
    assert!(
        !handler_imports.is_empty(),
        "LifecycleHandler should have imports"
    );
    assert!(
        handler_imports.iter().any(|i| i.contains("ExecAction")),
        "LifecycleHandler should import ExecAction"
    );
    assert!(
        handler_imports.iter().any(|i| i.contains("HTTPGetAction")),
        "LifecycleHandler should import HTTPGetAction"
    );

    // HTTPGetAction should import IntOrString from v0
    let http_imports = import_map.get_imports_for("HTTPGetAction");
    println!("HTTPGetAction imports: {:?}", http_imports);
    println!("Import map debug: {:?}", import_map);
    assert!(
        http_imports
            .iter()
            .any(|i| i.contains("../v0/intorstring.ncl")),
        "HTTPGetAction should import IntOrString from v0"
    );
    Ok(())
}

#[test]
fn test_single_module_generation() -> Result<(), Box<dyn std::error::Error>> {
    // Test that PackageWalkerAdapter creates a single module for all types
    let mut types = HashMap::new();

    for i in 1..=5 {
        let type_def = TypeDefinition {
            name: format!("Type{}", i),
            ty: Type::String,
            documentation: None,
            annotations: Default::default(),
        };
        types.insert(format!("Type{}", i), type_def);
    }

    let registry = PackageWalkerAdapter::build_registry(&types, "test.io", "v1")
        ?;
    let deps = PackageWalkerAdapter::build_dependencies(&registry);
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "test.io", "v1")
        ?;

    // Should have exactly one module
    assert_eq!(ir.modules.len(), 1, "Should generate exactly one module");
    assert_eq!(
        ir.modules[0].name, "test.io.v1",
        "Module name should be test.io.v1"
    );
    assert_eq!(
        ir.modules[0].types.len(),
        5,
        "Module should contain all 5 types"
    );
    Ok(())
}

#[test]
fn test_cross_module_import_generation() -> Result<(), Box<dyn std::error::Error>> {
    // Test imports between different versions

    // v1 type that references v0 type
    let v1_type = TypeDefinition {
        name: "Container".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = BTreeMap::new();
                fields.insert(
                    "port".to_string(),
                    Field {
                        ty: Type::Reference {
                            name: "IntOrString".to_string(),
                            module: Some("k8s.io.v0".to_string()),
                        },
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
    };

    let mut v1_types = HashMap::new();
    v1_types.insert("Container".to_string(), v1_type);

    // Build and generate
    let registry = PackageWalkerAdapter::build_registry(&v1_types, "k8s.io", "v1")
        ?;
    let deps = PackageWalkerAdapter::build_dependencies(&registry);
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "k8s.io", "v1")
        ?;

    // Generate Nickel code
    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let (_output, import_map) = codegen
        .generate_with_import_tracking(&ir)
        ?;

    // Check imports
    let container_imports = import_map.get_imports_for("Container");
    assert!(
        container_imports
            .iter()
            .any(|i| i.contains("../v0/intorstring.ncl")),
        "Container should import IntOrString from v0"
    );
    Ok(())
}
