//! Integration tests for amalgam-parser

use amalgam_codegen::Codegen;
use amalgam_parser::{
    crd::{CRDParser, CRD},
    package::NamespacedPackage,
    Parser,
};
use tempfile::TempDir;

fn load_test_crd(yaml_content: &str) -> Result<CRD, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_str(yaml_content)?)
}

#[test]
fn test_end_to_end_crd_to_nickel() -> Result<(), Box<dyn std::error::Error>> {
    let crd_yaml = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: compositions.apiextensions.crossplane.io
spec:
  group: apiextensions.crossplane.io
  names:
    kind: Composition
    plural: compositions
    singular: composition
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        required:
        - spec
        properties:
          spec:
            type: object
            required:
            - resources
            properties:
              resources:
                type: array
                items:
                  type: object
                  properties:
                    name:
                      type: string
                    base:
                      type: object
              compositeTypeRef:
                type: object
                properties:
                  apiVersion:
                    type: string
                  kind:
                    type: string
"#;

    let crd = load_test_crd(crd_yaml)?;
    let parser = CRDParser::new();
    let ir = parser.parse(crd.clone())?;

    // Verify IR was generated with one module for the single version
    assert_eq!(
        ir.modules.len(),
        1,
        "Should have 1 module for single version"
    );
    assert!(ir.modules[0].name.contains("Composition"));
    assert!(ir.modules[0].name.contains("v1"));

    // Generate Nickel code
    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let nickel_code = codegen.generate(&ir)?;

    // Verify generated code contains expected elements
    // With single-type module optimization, the type is exported directly
    // So we check for the fields rather than the type name wrapper
    assert!(
        nickel_code.contains("spec"),
        "Missing spec in generated code"
    );
    assert!(
        nickel_code.contains("resources"),
        "Missing resources in generated code"
    );
    assert!(
        nickel_code.contains("compositeTypeRef"),
        "Missing compositeTypeRef in generated code"
    );
    Ok(())
}

#[test]
fn test_package_structure_generation() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let _output_path = temp_dir.path().to_path_buf();

    // Use unified pipeline with NamespacedPackage
    let mut package = NamespacedPackage::new("test-package".to_string());
    let parser = CRDParser::new();

    // CRD definitions
    let crd1_yaml = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: widgets.example.io
spec:
  group: example.io
  names:
    kind: Widget
    plural: widgets
    singular: widget
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
"#;

    let crd2_yaml = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: gadgets.example.io
spec:
  group: example.io
  names:
    kind: Gadget
    plural: gadgets
    singular: gadget
  versions:
  - name: v1
    served: true
    storage: false
    schema:
      openAPIV3Schema:
        type: object
  - name: v2
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
"#;

    // Parse and add CRDs to package
    for crd_yaml in [crd1_yaml, crd2_yaml] {
        let crd = load_test_crd(crd_yaml)?;
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

    // Verify package structure
    assert_eq!(package.groups().len(), 1);
    assert!(package.groups().contains(&"example.io".to_string()));

    let versions = package.versions("example.io");
    assert!(versions.contains(&"v1".to_string()));
    assert!(versions.contains(&"v2".to_string()));

    let v1_kinds = package.kinds("example.io", "v1");
    assert!(v1_kinds.contains(&"widget".to_string()));
    assert!(v1_kinds.contains(&"gadget".to_string()));

    let v2_kinds = package.kinds("example.io", "v2");
    assert!(v2_kinds.contains(&"gadget".to_string()));
    assert!(!v2_kinds.contains(&"widget".to_string()));
    Ok(())
}

#[test]
fn test_complex_schema_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let crd_yaml = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: complex.test.io
spec:
  group: test.io
  names:
    kind: Complex
    plural: complexes
    singular: complex
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              stringField:
                type: string
                default: "default-value"
              intField:
                type: integer
                minimum: 0
                maximum: 100
              arrayField:
                type: array
                items:
                  type: string
              mapField:
                type: object
                additionalProperties:
                  type: number
              nestedObject:
                type: object
                properties:
                  innerString:
                    type: string
                  innerBool:
                    type: boolean
              enumField:
                type: string
                enum:
                - value1
                - value2
                - value3
              unionField:
                oneOf:
                - type: string
                - type: number
              optionalField:
                type: string
                nullable: true
"#;

    let crd = load_test_crd(crd_yaml)?;
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    // Find the Complex type in the IR
    let complex_module = ir
        .modules
        .iter()
        .find(|m| m.name.contains("Complex"))
        .ok_or("Module not found")?;

    let complex_type = complex_module
        .types
        .iter()
        .find(|t| t.name == "Complex")
        .ok_or("Module not found")?;

    // Verify the type structure
    match &complex_type.ty {
        amalgam_core::types::Type::Record { fields, .. } => {
            assert!(fields.contains_key("spec"));
            // Further nested validation could be done here
        }
        _ => return Err("Expected Complex to be a Record type".into()),
    }
    Ok(())
}

#[test]
fn test_multi_version_crd() -> Result<(), Box<dyn std::error::Error>> {
    let crd_yaml = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: multiversion.test.io
spec:
  group: test.io
  names:
    kind: MultiVersion
    plural: multiversions
    singular: multiversion
  versions:
  - name: v1alpha1
    served: true
    storage: false
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              alphaField:
                type: string
  - name: v1beta1
    served: true
    storage: false
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              alphaField:
                type: string
              betaField:
                type: integer
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              alphaField:
                type: string
              betaField:
                type: integer
              stableField:
                type: boolean
"#;

    let crd = load_test_crd(crd_yaml)?;
    let parser = CRDParser::new();
    let ir = parser.parse(crd.clone())?;

    // Parser should create separate modules for each version
    assert_eq!(ir.modules.len(), 3, "Should have 3 modules for 3 versions");

    // Check that each version has its own module
    let module_names: Vec<String> = ir.modules.iter().map(|m| m.name.clone()).collect();

    assert!(
        module_names.iter().any(|n| n.contains("v1alpha1")),
        "Should have v1alpha1 module"
    );
    assert!(
        module_names.iter().any(|n| n.contains("v1beta1")),
        "Should have v1beta1 module"
    );
    assert!(
        module_names.iter().any(|n| n.contains(".v1.")),
        "Should have v1 module"
    );

    // Each module should have the MultiVersion type
    for module in &ir.modules {
        assert_eq!(module.types.len(), 1, "Each module should have one type");
        assert_eq!(module.types[0].name, "MultiVersion");
    }
    Ok(())
}

#[test]
fn test_multi_version_package_generation() -> Result<(), Box<dyn std::error::Error>> {
    let crd_yaml = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: evolving.test.io
spec:
  group: test.io
  names:
    kind: Evolving
    plural: evolvings
    singular: evolving
  versions:
  - name: v1alpha1
    served: true
    storage: false
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              alphaField:
                type: string
  - name: v1beta1
    served: true
    storage: false
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              alphaField:
                type: string
              betaField:
                type: integer
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              alphaField:
                type: string
              betaField:
                type: integer
              stableField:
                type: boolean
"#;

    let _temp_dir = tempfile::TempDir::new()?;
    // Use unified pipeline with NamespacedPackage
    let mut package = NamespacedPackage::new("evolution-test".to_string());
    let parser = CRDParser::new();

    let crd = load_test_crd(crd_yaml)?;
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

    // Verify all versions are present
    let versions = package.versions("test.io");
    assert_eq!(versions.len(), 3, "Should have 3 versions");
    assert!(versions.contains(&"v1alpha1".to_string()));
    assert!(versions.contains(&"v1beta1".to_string()));
    assert!(versions.contains(&"v1".to_string()));

    // Each version should have the evolving kind
    for version in &["v1alpha1", "v1beta1", "v1"] {
        let kinds = package.kinds("test.io", version);
        assert_eq!(kinds.len(), 1, "Each version should have 1 kind");
        assert!(kinds.contains(&"evolving".to_string()));
    }

    // Verify we can generate files for each version
    let v1alpha1_files = package.generate_version_files("test.io", "v1alpha1");
    // Type names are PascalCase, so the file should be "Evolving.ncl"
    assert!(
        v1alpha1_files.contains_key("Evolving.ncl"),
        "Missing Evolving.ncl in v1alpha1 files"
    );

    let v1beta1_files = package.generate_version_files("test.io", "v1beta1");
    assert!(
        v1beta1_files.contains_key("Evolving.ncl"),
        "Missing Evolving.ncl in v1beta1 files"
    );

    let v1_files = package.generate_version_files("test.io", "v1");
    assert!(
        v1_files.contains_key("Evolving.ncl"),
        "Missing Evolving.ncl in v1 files"
    );
    Ok(())
}

#[test]
fn test_crd_with_validation_rules() -> Result<(), Box<dyn std::error::Error>> {
    let crd_yaml = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: validated.test.io
spec:
  group: test.io
  names:
    kind: Validated
    plural: validateds
    singular: validated
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        required:
        - spec
        properties:
          spec:
            type: object
            required:
            - requiredField
            properties:
              requiredField:
                type: string
                minLength: 3
                maxLength: 10
                pattern: "^[a-z]+$"
              numberWithBounds:
                type: number
                minimum: 0.0
                maximum: 100.0
                exclusiveMinimum: true
              arrayWithLimits:
                type: array
                minItems: 1
                maxItems: 5
                uniqueItems: true
                items:
                  type: string
"#;

    let crd = load_test_crd(crd_yaml)?;
    let parser = CRDParser::new();
    let ir = parser.parse(crd)?;

    // Generate code and verify validation constraints are preserved
    let mut codegen = amalgam_codegen::nickel::NickelCodegen::from_ir(&ir);
    let nickel_code = codegen.generate(&ir)?;

    // Check that required fields are marked
    assert!(nickel_code.contains("requiredField"));
    // Note: Actual validation constraints would need to be implemented
    // in the code generator to be properly tested here
    Ok(())
}
