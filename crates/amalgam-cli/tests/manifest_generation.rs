//! Tests for manifest generation functionality

use amalgam::manifest::{ManifestConfig, ManifestGenerator, PackageDefinition, DependencySpec, SourceType};
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_dependency_spec_basics() {
    // Test simple version
    let simple = DependencySpec::Simple("1.2.3".to_string());
    match simple {
        DependencySpec::Simple(v) => assert_eq!(v, "1.2.3"),
        _ => panic!("Expected Simple dependency spec"),
    }
    
    // Test full dependency spec
    let full = DependencySpec::Full { 
        version: "2.0.0".to_string(), 
        min_version: Some("1.0.0".to_string()) 
    };
    match full {
        DependencySpec::Full { version, .. } => {
            assert_eq!(version, "2.0.0");
        }
        _ => panic!("Expected Full dependency spec"),
    }
}

#[test]
fn test_manifest_with_index_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    let output_base = temp_dir.path().to_path_buf();
    
    let config = ManifestConfig {
        output_base: output_base.clone(),
        base_package_id: "github:test/packages".to_string(),
        package_mode: true,
        local_package_prefix: None, // No local prefix - should always use Index
    };
    
    let packages = vec![
        PackageDefinition {
            name: "k8s-io".to_string(),
            output: "k8s_io".to_string(),
            source_type: SourceType::Url,
            url: Some("https://github.com/kubernetes/kubernetes".to_string()),
            file: None,
            version: Some("1.31.0".to_string()),
            git_ref: Some("v1.31.0".to_string()),
            description: "Kubernetes core types".to_string(),
            keywords: vec!["kubernetes".to_string()],
            dependencies: HashMap::new(),
            enabled: true,
        },
        PackageDefinition {
            name: "crossplane".to_string(),
            output: "crossplane".to_string(),
            source_type: SourceType::Url,
            url: Some("https://github.com/crossplane/crossplane".to_string()),
            file: None,
            version: Some("1.17.2".to_string()),
            git_ref: Some("v1.17.2".to_string()),
            description: "Crossplane CRDs".to_string(),
            keywords: vec!["crossplane".to_string()],
            dependencies: {
                let mut deps = HashMap::new();
                deps.insert("k8s_io".to_string(), DependencySpec::Simple("1.31.0".to_string()));
                deps
            },
            enabled: true,
        },
    ];
    
    let generator = ManifestGenerator::new(config, packages);
    
    // Create dummy package directories
    fs::create_dir_all(output_base.join("k8s_io")).unwrap();
    fs::create_dir_all(output_base.join("crossplane")).unwrap();
    
    // Create dummy mod.ncl files with import
    fs::write(
        output_base.join("k8s_io/mod.ncl"),
        "{ test = \"k8s\" }"
    ).unwrap();
    
    fs::write(
        output_base.join("crossplane/mod.ncl"),
        "let k8s = import \"../k8s_io/mod.ncl\" in { test = \"crossplane\" }"
    ).unwrap();
    
    // Generate manifest for crossplane
    let crossplane_pkg = &generator.packages[1];
    generator.generate_package_manifest(crossplane_pkg, &output_base.join("crossplane")).unwrap();
    
    // Read and verify the generated manifest
    let manifest_path = output_base.join("crossplane/Nickel-pkg.ncl");
    assert!(manifest_path.exists(), "Manifest should be created");
    
    let manifest_content = fs::read_to_string(manifest_path).unwrap();
    
    // Verify Index dependency format
    assert!(manifest_content.contains("'Index { package = \"github:test/packages/k8s-io\", version = \"1.31.0\" }"),
            "Should use Index dependency format");
    
    // Should NOT contain Path dependencies
    assert!(!manifest_content.contains("'Path"),
            "Should not contain Path dependencies");
    
    // Verify metadata comments
    assert!(manifest_content.contains("# Generated:"), "Should have generation timestamp");
    assert!(manifest_content.contains("# Git ref: v1.17.2"), "Should have git ref");
    assert!(manifest_content.contains("# Generator: amalgam"), "Should have generator info");
}

#[test]
fn test_manifest_without_local_prefix() {
    let temp_dir = TempDir::new().unwrap();
    let output_base = temp_dir.path().to_path_buf();
    
    let config = ManifestConfig {
        output_base: output_base.clone(),
        base_package_id: "github:org/repo".to_string(),
        package_mode: true,
        local_package_prefix: Some("examples/packages".to_string()), // Even with prefix, should use Index
    };
    
    let packages = vec![
        PackageDefinition {
            name: "pkg-a".to_string(),
            output: "pkg_a".to_string(),
            source_type: "local".to_string(),
            url: None,
            path: Some("pkg_a".to_string()),
            version: Some("1.0.0".to_string()),
            git_ref: None,
            dependencies: HashMap::new(),
        },
        PackageDefinition {
            name: "pkg-b".to_string(),
            output: "pkg_b".to_string(),
            source_type: "local".to_string(),
            url: None,
            path: Some("pkg_b".to_string()),
            version: Some("1.0.0".to_string()),
            git_ref: None,
            dependencies: {
                let mut deps = HashMap::new();
                deps.insert("pkg_a".to_string(), DependencySpec::Simple("1.0.0".to_string()));
                deps
            },
        },
    ];
    
    let generator = ManifestGenerator::new(config, packages);
    
    // Create package structure
    fs::create_dir_all(output_base.join("pkg_a")).unwrap();
    fs::create_dir_all(output_base.join("pkg_b")).unwrap();
    
    fs::write(
        output_base.join("pkg_a/mod.ncl"),
        "{ test = \"a\" }"
    ).unwrap();
    
    fs::write(
        output_base.join("pkg_b/mod.ncl"),
        "let a = import \"../pkg_a/mod.ncl\" in { test = \"b\" }"
    ).unwrap();
    
    // Generate manifest for pkg_b
    let pkg_b = &generator.packages[1];
    generator.generate_package_manifest(pkg_b, &output_base.join("pkg_b")).unwrap();
    
    let manifest_content = fs::read_to_string(output_base.join("pkg_b/Nickel-pkg.ncl")).unwrap();
    
    // Should always use Index dependencies, never Path
    assert!(manifest_content.contains("'Index { package = \"github:org/repo/pkg-a\", version = \"1.0.0\" }"),
            "Should use Index dependency even for local packages");
    assert!(!manifest_content.contains("'Path"),
            "Should never use Path dependencies");
}

#[test]
fn test_auto_detect_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    let output_base = temp_dir.path().to_path_buf();
    
    let config = ManifestConfig {
        output_base: output_base.clone(),
        base_package_id: "github:test/pkgs".to_string(),
        package_mode: true,
        local_package_prefix: None,
    };
    
    let packages = vec![
        PackageDefinition {
            name: "base".to_string(),
            output: "base".to_string(),
            source_type: "local".to_string(),
            url: None,
            path: Some("base".to_string()),
            version: Some("1.0.0".to_string()),
            git_ref: None,
            dependencies: HashMap::new(),
        },
        PackageDefinition {
            name: "derived".to_string(),
            output: "derived".to_string(),
            source_type: "local".to_string(),
            url: None,
            path: Some("derived".to_string()),
            version: Some("2.0.0".to_string()),
            git_ref: None,
            dependencies: HashMap::new(), // No explicit deps - should auto-detect
        },
    ];
    
    let generator = ManifestGenerator::new(config, packages);
    
    // Create package with import that should be auto-detected
    fs::create_dir_all(output_base.join("base")).unwrap();
    fs::create_dir_all(output_base.join("derived")).unwrap();
    
    fs::write(
        output_base.join("base/mod.ncl"),
        "{ BaseType = { field | String } }"
    ).unwrap();
    
    // Import from base - this should be auto-detected
    fs::write(
        output_base.join("derived/mod.ncl"),
        r#"let base = import "../base/mod.ncl" in
        {
          DerivedType = base.BaseType & { extra | Number }
        }"#
    ).unwrap();
    
    let derived_pkg = &generator.packages[1];
    generator.generate_package_manifest(derived_pkg, &output_base.join("derived")).unwrap();
    
    let manifest_content = fs::read_to_string(output_base.join("derived/Nickel-pkg.ncl")).unwrap();
    
    // Should auto-detect and add base as dependency with Index format
    assert!(manifest_content.contains("dependencies = {"),
            "Should have dependencies section");
    assert!(manifest_content.contains("base = 'Index { package = \"github:test/pkgs/base\""),
            "Should auto-detect base dependency with Index format");
}

#[test]
fn test_manifest_with_git_ref() {
    let temp_dir = TempDir::new().unwrap();
    let output_base = temp_dir.path().to_path_buf();
    
    let config = ManifestConfig {
        output_base: output_base.clone(),
        base_package_id: "github:example/packages".to_string(),
        package_mode: true,
        local_package_prefix: None,
    };
    
    let packages = vec![
        PackageDefinition {
            name: "versioned-pkg".to_string(),
            output: "versioned_pkg".to_string(),
            source_type: "url".to_string(),
            url: Some("https://github.com/example/repo".to_string()),
            path: None,
            version: Some("3.2.1".to_string()),
            git_ref: Some("v3.2.1".to_string()),
            dependencies: HashMap::new(),
        },
    ];
    
    let generator = ManifestGenerator::new(config, packages);
    
    fs::create_dir_all(output_base.join("versioned_pkg")).unwrap();
    fs::write(
        output_base.join("versioned_pkg/mod.ncl"),
        "{ version = \"3.2.1\" }"
    ).unwrap();
    
    let pkg = &generator.packages[0];
    generator.generate_package_manifest(pkg, &output_base.join("versioned_pkg")).unwrap();
    
    let manifest_content = fs::read_to_string(output_base.join("versioned_pkg/Nickel-pkg.ncl")).unwrap();
    
    // Verify git ref is in metadata comments
    assert!(manifest_content.contains("# Git ref: v3.2.1"),
            "Should include git ref in metadata comments");
    
    // Verify version field
    assert!(manifest_content.contains("version = \"3.2.1\""),
            "Should have version field");
}

#[test]
fn test_package_resolution_from_manifest() {
    let temp_dir = TempDir::new().unwrap();
    let output_base = temp_dir.path().to_path_buf();
    
    let config = ManifestConfig {
        output_base: output_base.clone(),
        base_package_id: "github:myorg/packages".to_string(),
        package_mode: true,
        local_package_prefix: None,
    };
    
    // Test that package names are resolved correctly from manifest
    let packages = vec![
        PackageDefinition {
            name: "special-name".to_string(),
            output: "different_output".to_string(), // Different output dir
            source_type: "local".to_string(),
            url: None,
            path: Some("different_output".to_string()),
            version: Some("1.0.0".to_string()),
            git_ref: None,
            dependencies: HashMap::new(),
        },
        PackageDefinition {
            name: "consumer".to_string(),
            output: "consumer".to_string(),
            source_type: "local".to_string(),
            url: None,
            path: Some("consumer".to_string()),
            version: Some("1.0.0".to_string()),
            git_ref: None,
            dependencies: {
                let mut deps = HashMap::new();
                // Reference by output name
                deps.insert("different_output".to_string(), DependencySpec::Simple("1.0.0".to_string()));
                deps
            },
        },
    ];
    
    let generator = ManifestGenerator::new(config, packages);
    
    fs::create_dir_all(output_base.join("different_output")).unwrap();
    fs::create_dir_all(output_base.join("consumer")).unwrap();
    
    fs::write(
        output_base.join("different_output/mod.ncl"),
        "{ test = \"special\" }"
    ).unwrap();
    
    fs::write(
        output_base.join("consumer/mod.ncl"),
        "let special = import \"../different_output/mod.ncl\" in { test = \"consumer\" }"
    ).unwrap();
    
    let consumer_pkg = &generator.packages[1];
    generator.generate_package_manifest(consumer_pkg, &output_base.join("consumer")).unwrap();
    
    let manifest_content = fs::read_to_string(output_base.join("consumer/Nickel-pkg.ncl")).unwrap();
    
    // Should resolve to the package name, not output name
    assert!(manifest_content.contains("'Index { package = \"github:myorg/packages/special-name\""),
            "Should use package name (special-name) not output name in Index dependency");
}

#[cfg(test)]
mod validation_tests {
    use super::*;
    use std::process::Command;
    
    #[test]
    #[ignore] // Ignore by default as it requires nickel binary
    fn test_validate_generated_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let output_base = temp_dir.path().to_path_buf();
        
        let config = ManifestConfig {
            output_base: output_base.clone(),
            base_package_id: "github:test/packages".to_string(),
            package_mode: true,
            local_package_prefix: None,
        };
        
        let packages = vec![
            PackageDefinition {
                name: "simple".to_string(),
                output: "simple".to_string(),
                source_type: "local".to_string(),
                url: None,
                path: Some("simple".to_string()),
                version: Some("1.0.0".to_string()),
                git_ref: None,
                dependencies: HashMap::new(),
            },
        ];
        
        let generator = ManifestGenerator::new(config, packages);
        
        fs::create_dir_all(output_base.join("simple")).unwrap();
        fs::write(
            output_base.join("simple/mod.ncl"),
            "{ SimpleType = { name | String, value | Number } }"
        ).unwrap();
        
        let pkg = &generator.packages[0];
        generator.generate_package_manifest(pkg, &output_base.join("simple")).unwrap();
        
        // Try to validate with nickel if available
        let output = Command::new("nickel")
            .arg("typecheck")
            .arg(output_base.join("simple/Nickel-pkg.ncl"))
            .output();
        
        if let Ok(result) = output {
            assert!(result.status.success(), 
                    "Generated manifest should be valid Nickel: {}",
                    String::from_utf8_lossy(&result.stderr));
        }
    }
}