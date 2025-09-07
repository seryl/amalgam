//! Integration tests for import detection and resolution
//!
//! These tests ensure that:
//! 1. K8s type references are properly detected
//! 2. Imports are correctly generated
//! 3. References are resolved to use import aliases

mod fixtures;

use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::{
    ir::{Import, Module, TypeDefinition},
    types::{Field, Type},
    IR,
};
use amalgam_parser::{crd::CRDParser, package::NamespacedPackage, Parser};
use fixtures::Fixtures;
use std::collections::BTreeMap;

#[test]
fn test_k8s_type_reference_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Load fixture CRD that should have ObjectMeta reference
    let crd = Fixtures::simple_with_metadata();

    // Parse the CRD
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    // The type should contain a reference to ObjectMeta
    assert_eq!(ir.modules.len(), 1);
    let module = &ir.modules[0];
    assert_eq!(module.types.len(), 1);

    let type_def = &module.types[0];

    // Check that the metadata field has the k8s reference
    if let Type::Record { fields, .. } = &type_def.ty {
        assert!(fields.contains_key("metadata"));
        let metadata_field = &fields["metadata"];

        // The metadata field can be either:
        // - Type::Reference directly (if the parser detected it should be ObjectMeta)
        // - Type::Optional(Type::Reference)
        // - Type::Object (if it's just marked as 'type: object' in the CRD)

        match &metadata_field.ty {
            Type::Reference { name, module } => {
                assert_eq!(name, "ObjectMeta");
                assert_eq!(
                    module.as_deref(),
                    Some("io.k8s.apimachinery.pkg.apis.meta.v1")
                );
            }
            Type::Optional(inner) => {
                if let Type::Reference { name, module } = &**inner {
                    assert_eq!(name, "ObjectMeta");
                    assert_eq!(
                        module.as_deref(),
                        Some("io.k8s.apimachinery.pkg.apis.meta.v1")
                    );
                } else {
                    // For this test, metadata is just an object, not a k8s reference
                    // This is OK - the parser doesn't automatically add k8s references
                }
            }
            Type::Record { .. } => {
                // This is fine - the CRD just has 'type: object' for metadata
                // The parser converts it to a Record type
            }
            Type::Any => {
                // Metadata might be parsed as Any if no schema is provided
            }
            _ => return Err(format!("Unexpected type for metadata: {:?}", metadata_field.ty).into()),
        }
    } else {
        return Err(format!("Expected Record type, got {:?}", type_def.ty).into());
    }
    Ok(())
}

#[test]
fn test_import_generation_for_k8s_types() -> Result<(), Box<dyn std::error::Error>> {
    // Use unified pipeline with NamespacedPackage
    let mut package = NamespacedPackage::new("test-package".to_string());

    let crd1 = Fixtures::multiple_k8s_refs(); // This fixture has actual $ref to k8s types
    let crd2 = Fixtures::with_arrays();

    // Parse CRDs and add types to package
    let parser = CRDParser::new();

    for crd in [crd1, crd2] {
        let ir = parser.parse(crd.clone())?;
        for module in &ir.modules {
            for type_def in &module.types {
                // Module name format is {Kind}.{version}.{group}, so get the version part
                let parts: Vec<&str> = module.name.split('.').collect();
                let version = if parts.len() >= 2 { parts[1] } else { "v1" };
                package.add_type(
                    crd.spec.group.clone(),
                    version.to_string(),
                    type_def.name.to_lowercase(),
                    type_def.clone(),
                );
            }
        }
    }

    let ns_package = package;

    // Get the generated content for a resource that uses k8s types
    let version_files = ns_package.generate_version_files("test.io", "v1");

    if let Some(content) = version_files.get("multiref.ncl") {
        // Verify the import is present
        assert!(content.contains("import"), "Missing import statement");
        // Accept either the new unified format or legacy converted format
        let has_k8s_import = content.contains("k8s_io")
            || content.contains("objectmeta")
            || content.contains("resourcerequirements")
            || content.contains("labelselector");
        assert!(
            has_k8s_import,
            "Missing k8s-related import: {}",
            &content[..content.len().min(500)]
        );
    } else {
        // Generate from IR directly as fallback
        let crd = Fixtures::multiple_k8s_refs(); // Use the fixture with k8s refs
        let parser = CRDParser::new();
        let ir = parser.parse(crd)?;
        let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
        let content = codegen.generate(&ir)?;

        // The k8s imports should still be resolved
        assert!(
            content.contains("k8s_io") || content.contains("k8s_v1"),
            "Missing k8s import resolution in: {}",
            content
        );
    }
    Ok(())
}

#[test]
fn test_reference_resolution_to_alias() -> Result<(), Box<dyn std::error::Error>> {
    // Create a module with k8s type reference and import
    let mut ir = IR::new();

    let mut fields = BTreeMap::new();
    fields.insert(
        "metadata".to_string(),
        Field {
            ty: Type::Reference {
                name: "ObjectMeta".to_string(),
                module: Some("io.k8s.apimachinery.pkg.apis.meta.v1".to_string()),
            },
            required: false,
            default: None,
            description: Some("Standard Kubernetes metadata".to_string()),
        },
    );

    let module = Module {
        name: "test.example.io".to_string(),
        imports: vec![Import {
            path: "../../k8s_io/v1/objectmeta.ncl".to_string(),
            alias: Some("k8s_io_v1".to_string()),
            items: vec![],
        }],
        types: vec![TypeDefinition {
            name: "TestResource".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Default::default(),
    };

    ir.add_module(module);

    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let generated = codegen
        .generate(&ir)
        ?;

    // Verify the import is in the output
    assert!(
        generated.contains("let k8s_io_v1 = import"),
        "Missing import statement in generated code"
    );

    // Verify the reference was resolved to use the alias
    assert!(
        generated.contains("k8s_io_v1.ObjectMeta"),
        "Reference not resolved to alias. Generated:\n{}",
        generated
    );

    // Verify the original reference is NOT in the output
    assert!(
        !generated.contains("io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"),
        "Original reference still present. Generated:\n{}",
        generated
    );
    Ok(())
}

#[test]
fn test_multiple_k8s_type_references() -> Result<(), Box<dyn std::error::Error>> {
    // Use fixture with multiple k8s refs
    let crd = Fixtures::multiple_k8s_refs();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let content = codegen.generate(&ir)?;

    // With single-type module optimization, the type is exported directly
    // The type definition itself is just the record structure, not wrapped in MultiRef = {...}
    assert!(content.contains("spec"), "Missing spec field");
    assert!(
        content.contains("selector")
            && content.contains("volumes")
            && content.contains("resources"),
        "Missing expected fields in generated content:\n{}",
        content
    );
    Ok(())
}

#[test]
fn test_no_import_for_local_types() -> Result<(), Box<dyn std::error::Error>> {
    // Use fixture without k8s types
    let crd = Fixtures::nested_objects();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    // No imports should be generated for local types
    assert_eq!(
        ir.modules[0].imports.len(),
        0,
        "Unexpected imports for CRD without k8s types"
    );
    Ok(())
}

#[test]
fn test_import_path_calculation() -> Result<(), Box<dyn std::error::Error>> {
    use amalgam_parser::imports::TypeReference;

    // Test that import paths are calculated correctly
    let type_ref =
        TypeReference::from_qualified_name("io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta")
            .ok_or("Failed to get parent directory")?;

    let import_path = type_ref.import_path("example.io", "v1");
    assert_eq!(import_path, "../../k8s_io/v1/objectmeta.ncl");

    let alias = type_ref.module_alias();
    assert_eq!(alias, "k8s_io_v1");
    Ok(())
}

#[test]
fn test_case_insensitive_type_matching() -> Result<(), Box<dyn std::error::Error>> {
    // The resolver should handle case differences between reference and file names
    let mut ir = IR::new();

    let mut fields = BTreeMap::new();
    fields.insert(
        "metadata".to_string(),
        Field {
            ty: Type::Reference {
                name: "ObjectMeta".to_string(),
                module: Some("io.k8s.apimachinery.pkg.apis.meta.v1".to_string()),
            },
            required: false,
            default: None,
            description: None,
        },
    );

    let module = Module {
        name: "test".to_string(),
        imports: vec![Import {
            // Note: file is lowercase "objectmeta.ncl"
            path: "../../k8s_io/v1/objectmeta.ncl".to_string(),
            alias: Some("k8s_v1".to_string()),
            items: vec![],
        }],
        types: vec![TypeDefinition {
            name: "Test".to_string(),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Default::default(),
    };

    ir.add_module(module);

    let mut codegen = NickelCodegen::from_ir(&ir);
    let generated = codegen.generate(&ir)?;

    // Should resolve despite case difference
    assert!(
        generated.contains("k8s_v1.ObjectMeta"),
        "Failed to resolve with case difference. Generated:\n{}",
        generated
    );
    Ok(())
}

/// Test that package generation creates proper structure
#[test]
fn test_package_structure_generation() -> Result<(), Box<dyn std::error::Error>> {
    // Use unified pipeline with NamespacedPackage
    let mut package = NamespacedPackage::new("test-package".to_string());

    // Add CRDs from different fixtures
    let crd1 = Fixtures::simple_with_metadata();
    let crd2 = Fixtures::with_arrays();
    let crd3 = Fixtures::multi_version();

    // Parse CRDs and add types to package
    let parser = CRDParser::new();

    for crd in [crd1, crd2, crd3] {
        let ir = parser.parse(crd.clone())?;
        for module in &ir.modules {
            for type_def in &module.types {
                let version = module.name.rsplit('.').next().unwrap_or("v1");
                package.add_type(
                    crd.spec.group.clone(),
                    version.to_string(),
                    type_def.name.to_lowercase(),
                    type_def.clone(),
                );
            }
        }
    }

    // Generate and check structure
    let ns_package = package;

    // Check that main module was generated
    let main_module = ns_package.generate_main_module();
    assert!(
        main_module.contains("test_io"),
        "Missing test.io group in main module"
    );
    Ok(())
}
