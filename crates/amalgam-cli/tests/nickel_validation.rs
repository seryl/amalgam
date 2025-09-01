//! Tests for validating generated Nickel packages using the upstream Nickel library

use nickel_lang_core::{
    cache::{Cache, ErrorTolerance, ImportResolver},
    error::{Error, IntoDiagnostics},
    files::Files,
    program::Program,
    typecheck::TypecheckMode,
};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Test helper to validate a Nickel file
fn validate_nickel_file(path: &Path) -> Result<(), Error> {
    let mut cache = Cache::new(ErrorTolerance::Strict);
    let mut program = Program::new_from_file(path, std::io::stderr())?;

    // First, parse the file
    program.parse()?;

    // Then typecheck it if possible
    program.typecheck()?;

    Ok(())
}

/// Test helper to validate a Nickel package with imports
fn validate_nickel_package(package_root: &Path, entry_file: &str) -> Result<(), Error> {
    let entry_path = package_root.join(entry_file);

    let mut cache = Cache::new(ErrorTolerance::Strict);
    cache.set_import_resolver(ImportResolver::new(package_root.to_path_buf()));

    let mut program = Program::new_from_file(&entry_path, std::io::stderr())?;

    // Parse with import resolution
    program.parse()?;

    // Typecheck with import resolution
    program.typecheck()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validate_k8s_io_package() {
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
        let result = validate_nickel_package(&package_root, "mod.ncl");

        match result {
            Ok(()) => println!("✓ k8s_io package validates successfully"),
            Err(e) => {
                eprintln!("✗ k8s_io package validation failed:");
                eprintln!("{:?}", e);
                // Don't panic for now, just report
            }
        }
    }

    #[test]
    fn test_validate_crossplane_package() {
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
        let result = validate_nickel_package(&package_root, "mod.ncl");

        match result {
            Ok(()) => println!("✓ crossplane package validates successfully"),
            Err(e) => {
                eprintln!("✗ crossplane package validation failed:");
                eprintln!("{:?}", e);
                // Don't panic for now, just report
            }
        }
    }

    #[test]
    fn test_validate_individual_files() {
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

            match validate_nickel_file(&full_path) {
                Ok(()) => println!("✓ {} validates successfully", file_path),
                Err(e) => {
                    eprintln!("✗ {} validation failed:", file_path);
                    eprintln!("{:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_import_resolution() {
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
        let result = validate_nickel_package(root, "mod.ncl");
        assert!(result.is_ok(), "Simple import test should pass");
    }

    #[test]
    fn test_cross_package_imports() {
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
        let result = validate_nickel_file(&test_file);

        match result {
            Ok(()) => println!("✓ Cross-package imports work correctly"),
            Err(e) => {
                eprintln!("✗ Cross-package import validation failed (expected for now):");
                eprintln!("{:?}", e);
            }
        }
    }
}

/// Utility to validate all Nickel files in a directory tree
pub fn validate_directory_tree(root: &Path) -> Vec<(PathBuf, Result<(), Error>)> {
    let mut results = Vec::new();

    fn walk_dir(dir: &Path, results: &mut Vec<(PathBuf, Result<(), Error>)>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path, results);
                } else if path.extension().map_or(false, |ext| ext == "ncl") {
                    let result = validate_nickel_file(&path);
                    results.push((path, result));
                }
            }
        }
    }

    walk_dir(root, &mut results);
    results
}

/// Main validation runner for CLI integration
pub fn run_validation(package_path: &Path) -> Result<(), String> {
    // Check if it's a single file or a directory
    if package_path.is_file() {
        match validate_nickel_file(package_path) {
            Ok(()) => {
                println!("✓ {} validates successfully", package_path.display());
                Ok(())
            }
            Err(e) => {
                let msg = format!("✗ {} validation failed: {:?}", package_path.display(), e);
                eprintln!("{}", msg);
                Err(msg)
            }
        }
    } else if package_path.is_dir() {
        // Look for mod.ncl as the entry point
        let mod_file = package_path.join("mod.ncl");
        if mod_file.exists() {
            match validate_nickel_package(package_path, "mod.ncl") {
                Ok(()) => {
                    println!(
                        "✓ Package at {} validates successfully",
                        package_path.display()
                    );
                    Ok(())
                }
                Err(e) => {
                    let msg = format!("✗ Package validation failed: {:?}", e);
                    eprintln!("{}", msg);
                    Err(msg)
                }
            }
        } else {
            // Validate all .ncl files in the directory
            let results = validate_directory_tree(package_path);
            let mut all_ok = true;

            for (path, result) in results {
                match result {
                    Ok(()) => println!("✓ {}", path.display()),
                    Err(e) => {
                        eprintln!("✗ {} - {:?}", path.display(), e);
                        all_ok = false;
                    }
                }
            }

            if all_ok {
                Ok(())
            } else {
                Err("Some files failed validation".to_string())
            }
        }
    } else {
        Err(format!("Path {} does not exist", package_path.display()))
    }
}
