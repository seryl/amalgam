//! Tests for Nickel package generation

use amalgam_codegen::nickel_package::{
    NickelPackageConfig, NickelPackageGenerator, PackageDependency,
};
use amalgam_parser::crd::{CRDMetadata, CRDNames, CRDSchema, CRDSpec, CRDVersion, CRD};
use amalgam_parser::package::PackageGenerator;
use std::path::PathBuf;

fn sample_crd() -> CRD {
    CRD {
        api_version: "apiextensions.k8s.io/v1".to_string(),
        kind: "CustomResourceDefinition".to_string(),
        metadata: CRDMetadata {
            name: "compositions.apiextensions.crossplane.io".to_string(),
        },
        spec: CRDSpec {
            group: "apiextensions.crossplane.io".to_string(),
            names: CRDNames {
                kind: "Composition".to_string(),
                plural: "compositions".to_string(),
                singular: "composition".to_string(),
            },
            versions: vec![CRDVersion {
                name: "v1".to_string(),
                served: true,
                storage: true,
                schema: Some(CRDSchema {
                    openapi_v3_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "spec": {
                                "type": "object",
                                "properties": {
                                    "compositeTypeRef": {
                                        "type": "object",
                                        "properties": {
                                            "apiVersion": {"type": "string"},
                                            "kind": {"type": "string"}
                                        }
                                    },
                                    "mode": {
                                        "type": "string",
                                        "default": "Pipeline"
                                    }
                                }
                            }
                        }
                    }),
                }),
            }],
        },
    }
}

#[test]
fn test_generate_basic_nickel_manifest() {
    let config = NickelPackageConfig {
        name: "test-package".to_string(),
        version: "1.0.0".to_string(),
        minimal_nickel_version: "1.9.0".to_string(),
        description: "A test package".to_string(),
        authors: vec!["Test Author".to_string()],
        license: "MIT".to_string(),
        keywords: vec!["test".to_string(), "example".to_string()],
    };

    let generator = NickelPackageGenerator::new(config);
    let manifest = generator
        .generate_manifest(&[], std::collections::HashMap::new())
        .unwrap();

    // Check that the manifest contains expected content
    assert!(manifest.contains("name = \"test-package\""));
    assert!(manifest.contains("version = \"1.0.0\""));
    assert!(manifest.contains("description = \"A test package\""));
    assert!(manifest.contains("authors = ["));
    assert!(manifest.contains("\"Test Author\""));
    assert!(manifest.contains("license = \"MIT\""));
    assert!(manifest.contains("keywords = ["));
    assert!(manifest.contains("\"test\""));
    assert!(manifest.contains("\"example\""));
    assert!(manifest.contains("minimal_nickel_version = \"1.9.0\""));
    assert!(manifest.contains("| std.package.Manifest"));
}

#[test]
fn test_nickel_manifest_with_dependencies() {
    let config = NickelPackageConfig::default();
    let generator = NickelPackageGenerator::new(config);

    let mut dependencies = std::collections::HashMap::new();
    dependencies.insert(
        "k8s_io".to_string(),
        PackageDependency::Path(PathBuf::from("../k8s_io")),
    );
    dependencies.insert(
        "stdlib".to_string(),
        PackageDependency::Index {
            package: "github:nickel-lang/stdlib".to_string(),
            version: ">=1.0.0".to_string(),
        },
    );

    let manifest = generator.generate_manifest(&[], dependencies).unwrap();

    assert!(manifest.contains("dependencies = {"));
    assert!(manifest.contains("k8s_io = 'Path \"../k8s_io\""));
    assert!(manifest.contains(
        "stdlib = 'Index { package = \"github:nickel-lang/stdlib\", version = \">=1.0.0\" }"
    ));
}

#[test]
fn test_package_generates_nickel_manifest() {
    let mut generator =
        PackageGenerator::new("test-crossplane".to_string(), PathBuf::from("/tmp/test"));

    generator.add_crd(sample_crd());
    let package = generator.generate_package().unwrap();

    let manifest = package.generate_nickel_manifest(None);

    // Check basic structure
    assert!(manifest.contains("name = \"test-crossplane\""));
    assert!(manifest.contains("description = \"Generated type definitions for test-crossplane\""));
    assert!(manifest.contains("version = \"0.1.0\""));
    assert!(manifest.contains("minimal_nickel_version = \"1.9.0\""));

    // Check that group-based keywords are added
    assert!(manifest.contains("\"apiextensions-crossplane-io\""));

    // Should detect k8s references if there are any
    // (in this simple test there aren't any)
    if manifest.contains("dependencies = {") {
        assert!(manifest.contains("k8s_io"));
    }

    assert!(manifest.contains("| std.package.Manifest"));
}

#[test]
fn test_dependency_formatting() {
    // Test Path dependency
    let path_dep = PackageDependency::Path(PathBuf::from("/some/path"));
    assert_eq!(path_dep.to_nickel_string(), "'Path \"/some/path\"");

    // Test Index dependency
    let index_dep = PackageDependency::Index {
        package: "github:org/repo".to_string(),
        version: "^1.0.0".to_string(),
    };
    assert_eq!(
        index_dep.to_nickel_string(),
        "'Index { package = \"github:org/repo\", version = \"^1.0.0\" }"
    );

    // Test Git dependency with branch
    let git_dep = PackageDependency::Git {
        url: "https://github.com/org/repo.git".to_string(),
        branch: Some("main".to_string()),
        tag: None,
        rev: None,
    };
    assert_eq!(
        git_dep.to_nickel_string(),
        "'Git { url = \"https://github.com/org/repo.git\", branch = \"main\" }"
    );

    // Test Git dependency with tag
    let git_tag_dep = PackageDependency::Git {
        url: "https://github.com/org/repo.git".to_string(),
        branch: None,
        tag: Some("v1.0.0".to_string()),
        rev: None,
    };
    assert_eq!(
        git_tag_dep.to_nickel_string(),
        "'Git { url = \"https://github.com/org/repo.git\", tag = \"v1.0.0\" }"
    );
}
