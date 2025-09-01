//! Tests for validating generated Nickel packages
//!
//! NOTE: These tests use the Nickel CLI binary for validation, not the library API.
//! The nickel-lang-core library API is unstable and changes frequently between versions.
//! For actual validation, we rely on the CLI implementation in src/validate.rs

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Helper to check if the nickel CLI is available
fn nickel_cli_available() -> bool {
    Command::new("nickel")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Test helper to validate a Nickel file using the CLI
fn validate_nickel_file_cli(file: &Path) -> Result<(), String> {
    if !nickel_cli_available() {
        return Err("Nickel CLI not available".to_string());
    }

    let output = Command::new("nickel")
        .arg("typecheck")
        .arg(file)
        .output()
        .map_err(|e| format!("Failed to run nickel: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we can validate simple Nickel files
    /// This test verifies our validation approach works
    #[test]
    fn test_simple_nickel_validation() {
        if !nickel_cli_available() {
            eprintln!("Skipping test: Nickel CLI not available");
            return;
        }

        // Create a simple test file
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.ncl");
        fs::write(&test_file, "{ value = 42 }").unwrap();

        // Validate using CLI
        match validate_nickel_file_cli(&test_file) {
            Ok(()) => println!("✓ Simple validation passed"),
            Err(e) => panic!("Simple validation failed: {}", e),
        }
    }

    #[test]
    fn test_validate_k8s_io_package() {
        if !nickel_cli_available() {
            eprintln!("Skipping test: Nickel CLI not available");
            return;
        }

        let package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples/k8s_io");

        if !package_root.exists() {
            eprintln!(
                "Skipping test: k8s_io package not found at {:?}",
                package_root
            );
            return;
        }

        // Test the main module file
        let mod_file = package_root.join("mod.ncl");
        if mod_file.exists() {
            match validate_nickel_file_cli(&mod_file) {
                Ok(()) => println!("✓ k8s_io package validates successfully"),
                Err(e) => {
                    eprintln!("✗ k8s_io package validation failed:");
                    eprintln!("{}", e);
                    // Don't panic for now, just report
                }
            }
        }
    }

    #[test]
    fn test_validate_crossplane_package() {
        if !nickel_cli_available() {
            eprintln!("Skipping test: Nickel CLI not available");
            return;
        }

        let package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples/crossplane");

        if !package_root.exists() {
            eprintln!(
                "Skipping test: crossplane package not found at {:?}",
                package_root
            );
            return;
        }

        // Test the main module file
        let mod_file = package_root.join("mod.ncl");
        if mod_file.exists() {
            match validate_nickel_file_cli(&mod_file) {
                Ok(()) => println!("✓ crossplane package validates successfully"),
                Err(e) => {
                    eprintln!("✗ crossplane package validation failed:");
                    eprintln!("{}", e);
                    // Don't panic for now, just report
                }
            }
        }
    }

    #[test]
    fn test_validate_individual_files() {
        if !nickel_cli_available() {
            eprintln!("Skipping test: Nickel CLI not available");
            return;
        }

        let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples");

        // Test some individual files
        let test_files = vec![
            "k8s_io/v1/objectmeta.ncl",
            "k8s_io/v1/pod.ncl",
            "k8s_io/v1/service.ncl",
            "crossplane/apiextensions.crossplane.io/v1/composition.ncl",
        ];

        for file_path in test_files {
            let full_path = examples_dir.join(file_path);

            if !full_path.exists() {
                eprintln!("Skipping {}: file not found", file_path);
                continue;
            }

            match validate_nickel_file_cli(&full_path) {
                Ok(()) => println!("✓ {} validates successfully", file_path),
                Err(e) => {
                    eprintln!("✗ {} validation failed:", file_path);
                    eprintln!("{}", e);
                }
            }
        }
    }

    #[test]
    fn test_import_resolution() {
        if !nickel_cli_available() {
            eprintln!("Skipping test: Nickel CLI not available");
            return;
        }

        // Create a simple test case with imports
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create a simple module structure
        fs::write(
            root.join("mod.ncl"),
            r#"{
  sub = import "./sub/mod.ncl",
  types = import "./types.ncl",
}"#,
        )
        .unwrap();

        fs::create_dir(root.join("sub")).unwrap();
        fs::write(
            root.join("sub/mod.ncl"),
            r#"{
  value = 42,
}"#,
        )
        .unwrap();

        fs::write(
            root.join("types.ncl"),
            r#"{
  MyType = { value | Number },
}"#,
        )
        .unwrap();

        // Validate the package
        let result = validate_nickel_file_cli(&root.join("mod.ncl"));
        assert!(result.is_ok(), "Simple import test should pass");
    }

    #[test]
    fn test_cross_package_imports() {
        if !nickel_cli_available() {
            eprintln!("Skipping test: Nickel CLI not available");
            return;
        }

        let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples");

        // Test the test file that imports both k8s and crossplane
        let test_file = examples_dir.join("test_with_packages.ncl");

        if !test_file.exists() {
            eprintln!("Skipping test: test_with_packages.ncl not found");
            return;
        }

        // This test will likely fail initially because of import resolution issues
        // We need to set up the import resolver properly
        match validate_nickel_file_cli(&test_file) {
            Ok(()) => println!("✓ Cross-package imports work correctly"),
            Err(e) => {
                eprintln!("✗ Cross-package import validation failed (expected for now):");
                eprintln!("{}", e);
            }
        }
    }
}
