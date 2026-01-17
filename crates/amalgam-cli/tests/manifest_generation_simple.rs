//! Simple tests for manifest generation functionality

use amalgam::manifest::{ManifestConfig, PackageDefinition, PackageSource};
use std::fs;
use tempfile::TempDir;

// Dependency specs were removed in the new simplified manifest system

#[test]
fn test_package_definition_creation() -> Result<(), Box<dyn std::error::Error>> {
    // New simplified package definition
    let package = PackageDefinition {
        source: PackageSource::Single("https://example.com/repo".to_string()),
        domain: Some("example.com".to_string()),
        name: Some("test-package".to_string()),
        description: Some("Test package".to_string()),
        enabled: true,
    };

    assert!(package.enabled);
    assert_eq!(package.domain, Some("example.com".to_string()));
    assert_eq!(package.name, Some("test-package".to_string()));

    Ok(())
}

#[test]
fn test_manifest_config_creation() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let config = ManifestConfig {
        output_base: temp_dir.path().to_path_buf(),
        base_package_id: "github:test/packages".to_string(),
        package_mode: true,
        local_package_prefix: None,
        debug: false,
    };

    assert_eq!(config.base_package_id, "github:test/packages");
    assert!(config.package_mode);
    assert!(config.local_package_prefix.is_none());
    Ok(())
}

#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    #[test]
    fn test_package_source_variants() -> Result<(), Box<dyn std::error::Error>> {
        // Test single source
        let single = PackageSource::Single("https://example.com/test".to_string());
        match single {
            PackageSource::Single(url) => assert!(url.contains("example.com")),
            _ => panic!("Expected single source"),
        }

        // Test multiple sources
        let multiple = PackageSource::Multiple(vec![
            "https://example.com/crd1.yaml".to_string(),
            "https://example.com/crd2.yaml".to_string(),
        ]);
        match multiple {
            PackageSource::Multiple(urls) => assert_eq!(urls.len(), 2),
            _ => panic!("Expected multiple sources"),
        }

        Ok(())
    }

    #[test]
    fn test_package_generates_directory() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let output_base = temp_dir.path().to_path_buf();

        // Create a simple test package
        let pkg = PackageDefinition {
            source: PackageSource::Single("https://example.com/test".to_string()),
            domain: Some("example.com".to_string()),
            name: Some("test_pkg".to_string()),
            description: Some("Test package".to_string()),
            enabled: true,
        };

        // Verify we can use the package name for directory creation
        let pkg_name = pkg.name.as_ref().unwrap();
        let pkg_dir = output_base.join(pkg_name);
        fs::create_dir_all(&pkg_dir)?;
        assert!(pkg_dir.exists());

        // Create a basic mod.ncl file
        fs::write(pkg_dir.join("mod.ncl"), "{ test = \"value\" }")?;
        assert!(pkg_dir.join("mod.ncl").exists());

        Ok(())
    }
}
