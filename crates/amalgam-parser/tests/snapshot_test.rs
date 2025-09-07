//! Snapshot tests for generated Nickel code
//!
//! These tests ensure that the generated output remains consistent
//! and catch any unintended changes to the code generation

mod fixtures;

use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_parser::{crd::CRDParser, package::NamespacedPackage, Parser};
use fixtures::Fixtures;
use insta::assert_snapshot;

#[test]
fn test_snapshot_simple_crd() -> Result<(), Box<dyn std::error::Error>> {
    let crd = Fixtures::simple_with_metadata();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let generated = codegen.generate(&ir)?;

    // Snapshot the generated code
    assert_snapshot!("simple_crd_nickel", generated);
    Ok(())
}

#[test]
fn test_snapshot_crd_with_k8s_imports() -> Result<(), Box<dyn std::error::Error>> {
    let crd = Fixtures::simple_with_metadata();
    let parser = CRDParser::new();
    let ir = parser.parse(crd.clone())?;

    // Use NamespacedPackage to handle imports (unified pipeline)
    let mut package = NamespacedPackage::new("test-package".to_string());

    // Add types from the parsed IR to the package
    for module in &ir.modules {
        for type_def in &module.types {
            // Extract version from module name
            let version = module.name.rsplit('.').next().unwrap_or("v1");
            package.add_type(
                crd.spec.group.clone(),
                version.to_string(),
                type_def.name.to_lowercase(),
                type_def.clone(),
            );
        }
    }

    let generated_package = package;

    // Get the generated content using the new batch generation
    let version_files = generated_package.generate_version_files("test.io", "v1");
    let content = if let Some(content) = version_files.get("simple.ncl") {
        content.clone()
    } else {
        // If no file found, generate from IR directly
        let mut codegen = NickelCodegen::from_ir(&ir);
        codegen.generate(&ir)?
    };

    // Snapshot should include imports and resolved references
    assert_snapshot!("simple_with_k8s_imports", content);
    Ok(())
}

#[test]
fn test_snapshot_multiple_k8s_refs() -> Result<(), Box<dyn std::error::Error>> {
    let crd = Fixtures::multiple_k8s_refs();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    let mut codegen = NickelCodegen::from_ir(&ir);
    let content = codegen.generate(&ir)?;

    assert_snapshot!("multiple_k8s_refs_nickel", content);
    Ok(())
}

#[test]
fn test_snapshot_nested_objects() -> Result<(), Box<dyn std::error::Error>> {
    let crd = Fixtures::nested_objects();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    let mut codegen = NickelCodegen::from_ir(&ir);
    let generated = codegen.generate(&ir)?;

    assert_snapshot!("nested_objects_nickel", generated);
    Ok(())
}

#[test]
fn test_snapshot_arrays() -> Result<(), Box<dyn std::error::Error>> {
    let crd = Fixtures::with_arrays();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    let mut codegen = NickelCodegen::from_ir(&ir);
    let content = codegen.generate(&ir)?;

    assert_snapshot!("arrays_nickel", content);
    Ok(())
}

#[test]
fn test_snapshot_validation() -> Result<(), Box<dyn std::error::Error>> {
    let crd = Fixtures::with_validation();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    let mut codegen = NickelCodegen::from_ir(&ir);
    let generated = codegen.generate(&ir)?;

    assert_snapshot!("validation_nickel", generated);
    Ok(())
}

#[test]
fn test_snapshot_multi_version() -> Result<(), Box<dyn std::error::Error>> {
    let crd = Fixtures::multi_version();
    let parser = CRDParser::new();

    // Parse all versions
    let ir = parser.parse(crd)?;

    // The IR should have modules for each version
    let mut codegen = NickelCodegen::from_ir(&ir);
    let all_versions = codegen.generate(&ir)?;

    // Snapshot the full multi-version output
    assert_snapshot!("multi_version_all", all_versions);
    Ok(())
}

#[test]
fn test_snapshot_ir_structure() -> Result<(), Box<dyn std::error::Error>> {
    // Also snapshot the IR structure to catch changes in parsing
    let crd = Fixtures::simple_with_metadata();
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    assert_snapshot!("simple_crd_ir", format!("{:#?}", ir));
    Ok(())
}

#[test]
fn test_snapshot_package_structure() -> Result<(), Box<dyn std::error::Error>> {
    let mut package = NamespacedPackage::new("test-package".to_string());

    // Add multiple CRDs using the unified pipeline
    for crd in [
        Fixtures::simple_with_metadata(),
        Fixtures::with_arrays(),
        Fixtures::multi_version(),
    ] {
        let parser = CRDParser::new();
        let ir = parser.parse(crd.clone())?;

        // Add types from the parsed IR to the package
        for module in &ir.modules {
            for type_def in &module.types {
                // Extract version from module name (e.g., "apiextensions.crossplane.io.v1" -> "v1")
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

    let ns_package = package;

    // Get the main module to see structure
    let main_module = ns_package.generate_main_module();

    assert_snapshot!("package_structure_main", main_module);
    Ok(())
}
