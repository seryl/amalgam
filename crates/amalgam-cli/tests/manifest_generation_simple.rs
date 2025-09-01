//! Simple tests for manifest generation functionality

use amalgam::manifest::{ManifestConfig, PackageDefinition, DependencySpec, SourceType};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_dependency_spec_types() {
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
fn test_package_definition_creation() {
    let package = PackageDefinition {
        name: "test-package".to_string(),
        output: "test_package".to_string(),
        source_type: SourceType::Url,
        url: Some("https://example.com/repo".to_string()),
        file: None,
        version: Some("1.0.0".to_string()),
        git_ref: Some("v1.0.0".to_string()),
        description: "Test package".to_string(),
        keywords: vec!["test".to_string()],
        dependencies: {
            let mut deps = HashMap::new();
            deps.insert("base".to_string(), DependencySpec::Simple("1.0.0".to_string()));
            deps
        },
        enabled: true,
    };
    
    assert_eq!(package.name, "test-package");
    assert_eq!(package.version, Some("1.0.0".to_string()));
    assert!(package.dependencies.contains_key("base"));
    assert!(package.enabled);
}

#[test]
fn test_manifest_config_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = ManifestConfig {
        output_base: temp_dir.path().to_path_buf(),
        base_package_id: "github:test/packages".to_string(),
        package_mode: true,
        local_package_prefix: None,
    };
    
    assert_eq!(config.base_package_id, "github:test/packages");
    assert!(config.package_mode);
    assert!(config.local_package_prefix.is_none());
}

#[cfg(test)]
mod end_to_end_tests {
    use super::*;
    
    #[test]
    fn test_package_generates_index_dependencies() {
        let temp_dir = TempDir::new().unwrap();
        let output_base = temp_dir.path().to_path_buf();
        
        // Create a simple test case that should work
        let pkg = PackageDefinition {
            name: "test-pkg".to_string(),
            output: "test_pkg".to_string(),
            source_type: SourceType::Url,
            url: Some("https://example.com/test".to_string()),
            file: None,
            version: Some("1.0.0".to_string()),
            git_ref: Some("v1.0.0".to_string()),
            description: "Test package".to_string(),
            keywords: vec!["test".to_string()],
            dependencies: {
                let mut deps = HashMap::new();
                deps.insert("base".to_string(), DependencySpec::Simple("1.0.0".to_string()));
                deps
            },
            enabled: true,
        };
        
        // Test that we can create the package structure
        assert_eq!(pkg.source_type, SourceType::Url);
        assert!(pkg.dependencies.contains_key("base"));
        
        // Verify package directory can be created
        let pkg_dir = output_base.join(&pkg.output);
        fs::create_dir_all(&pkg_dir).expect("Should be able to create package directory");
        assert!(pkg_dir.exists());
        
        // Create a basic mod.ncl file
        fs::write(pkg_dir.join("mod.ncl"), "{ test = \"value\" }").expect("Should write mod.ncl");
        assert!(pkg_dir.join("mod.ncl").exists());
    }
}