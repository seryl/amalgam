//! Test same-version imports within a package (e.g., v1alpha3 DeviceSelector case)

use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::{
    ir::TypeDefinition,
    types::{Field, Type},
    ImportPathCalculator,
};
use amalgam_parser::package_walker::PackageWalkerAdapter;
use std::collections::{BTreeMap, HashMap};

/// Create test types that reference each other within the same version
fn create_v1alpha3_types() -> HashMap<String, TypeDefinition> {
    let mut types = HashMap::new();

    // CELDeviceSelector type
    types.insert(
        "celdeviceselector".to_string(),
        TypeDefinition {
            name: "CELDeviceSelector".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "expression".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::String)),
                            required: false,
                            description: Some("CEL expression for device selection".to_string()),
                            default: None,
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: Some("CEL expression for selecting devices".to_string()),
            annotations: Default::default(),
        },
    );

    // DeviceSelector that references CELDeviceSelector
    types.insert(
        "deviceselector".to_string(),
        TypeDefinition {
            name: "DeviceSelector".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "cel".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::Reference {
                                name: "CELDeviceSelector".to_string(),
                                module: Some("k8s.io.v1alpha3".to_string()),
                            })),
                            required: false,
                            description: Some("CEL selector".to_string()),
                            default: None,
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: Some("Device selector with CEL support".to_string()),
            annotations: Default::default(),
        },
    );

    types
}

#[test]
fn test_v1alpha3_same_version_imports() {
    let types = create_v1alpha3_types();

    // Build registry and dependencies
    let registry = PackageWalkerAdapter::build_registry(&types, "k8s.io", "v1alpha3")
        .expect("Should build registry");
    let deps = PackageWalkerAdapter::build_dependencies(&registry);

    // Generate IR
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "k8s.io", "v1alpha3")
        .expect("Should generate IR");

    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let nickel_code = codegen.generate(&ir).expect("Should generate Nickel");

    // Verify deviceselector.ncl has correct import
    // The import should be "./celdeviceselector.ncl" for same version
    assert!(
        nickel_code.contains("./celdeviceselector"),
        "DeviceSelector should import CELDeviceSelector from same directory"
    );

    // Should not have cross-version imports
    assert!(
        !nickel_code.contains("../v1alpha3/"),
        "Should not have cross-version import to same version"
    );
}

#[test]
fn test_import_path_calculator_same_version() {
    let calc = ImportPathCalculator::new_standalone();

    // Test the specific v1alpha3 case
    let path = calc.calculate(
        "k8s.io",
        "v1alpha3",
        "k8s.io",
        "v1alpha3",
        "celdeviceselector",
    );
    assert_eq!(path, "./celdeviceselector.ncl");

    // Test other same-version cases
    let path = calc.calculate("k8s.io", "v1", "k8s.io", "v1", "pod");
    assert_eq!(path, "./pod.ncl");

    let path = calc.calculate(
        "crossplane.io",
        "v1beta1",
        "crossplane.io",
        "v1beta1",
        "composition",
    );
    assert_eq!(path, "./composition.ncl");
}

#[test]
fn test_same_version_multiple_references() {
    // Test a more complex scenario with multiple same-version references
    let mut types = HashMap::new();

    // Type A references B and C
    types.insert(
        "typea".to_string(),
        TypeDefinition {
            name: "TypeA".to_string(),
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "b_ref".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "TypeB".to_string(),
                                module: Some("test.io.v1".to_string()),
                            },
                            required: true,
                            description: None,
                            default: None,
                        },
                    );
                    fields.insert(
                        "c_ref".to_string(),
                        Field {
                            ty: Type::Optional(Box::new(Type::Reference {
                                name: "TypeC".to_string(),
                                module: Some("test.io.v1".to_string()),
                            })),
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
        },
    );

    // Type B (referenced by A)
    types.insert(
        "typeb".to_string(),
        TypeDefinition {
            name: "TypeB".to_string(),
            ty: Type::String,
            documentation: None,
            annotations: Default::default(),
        },
    );

    // Type C (referenced by A)
    types.insert(
        "typec".to_string(),
        TypeDefinition {
            name: "TypeC".to_string(),
            ty: Type::Number,
            documentation: None,
            annotations: Default::default(),
        },
    );

    // Generate IR
    let registry = PackageWalkerAdapter::build_registry(&types, "test.io", "v1")
        .expect("Should build registry");
    let deps = PackageWalkerAdapter::build_dependencies(&registry);
    let ir = PackageWalkerAdapter::generate_ir(registry, deps, "test.io", "v1")
        .expect("Should generate IR");

    // Check that TypeA module has local imports for B and C
    for module in &ir.modules {
        if module.name.contains("typea") {
            // Should not have any "../" imports for same version
            for import in &module.imports {
                assert!(
                    !import.path.contains("../"),
                    "Same version import should not go up directories: {}",
                    import.path
                );

                // Should use ./ for local imports
                if import.path.contains("typeb") || import.path.contains("typec") {
                    assert!(
                        import.path.starts_with("./"),
                        "Same version import should use ./: {}",
                        import.path
                    );
                }
            }
        }
    }
}
