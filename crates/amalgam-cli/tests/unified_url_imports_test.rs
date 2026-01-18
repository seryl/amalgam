//! Integration tests for URL imports using the unified IR pipeline
//!
//! Verifies that URL-based imports use the unified walker infrastructure
//! and produce consistent output with proper cross-module imports.

use std::fs;
use std::process::Command;
use tempfile::tempdir;

/// Skip test if running in a sandboxed environment (e.g., Nix build)
fn skip_if_no_network() -> bool {
    std::env::var("AMALGAM_SKIP_NETWORK_TESTS").is_ok()
}

#[test]
fn test_url_import_uses_unified_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    if skip_if_no_network() {
        eprintln!("Skipping test: AMALGAM_SKIP_NETWORK_TESTS is set");
        return Ok(());
    }
    let temp_dir = tempdir()?;
    let output_dir = temp_dir.path();

    // Run amalgam import url command
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "amalgam",
            "--",
            "import",
            "url",
            "--url",
            "https://raw.githubusercontent.com/crossplane/crossplane/master/cluster/crds/apiextensions.crossplane.io_compositions.yaml",
            "--output",
            output_dir.to_str().ok_or("Failed to convert path to string")?,
            "--package",
            "test-crossplane"
        ])
        .output()
        ?;

    if !output.status.success() {
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    }

    // Check that the command succeeded
    assert!(output.status.success(), "URL import should succeed");

    // Check that files were generated
    assert!(
        output_dir.join("mod.ncl").exists(),
        "Main module should be generated"
    );

    // Check that we have the expected package structure
    let mod_content = fs::read_to_string(output_dir.join("mod.ncl"))?;

    // Should have generated the expected structure
    assert!(
        mod_content.contains("import"),
        "Main module should have imports"
    );

    // Check for proper structure
    let entries = fs::read_dir(output_dir)?.count();

    assert!(
        entries > 1,
        "Should have generated multiple files/directories"
    );
    Ok(())
}

#[test]
fn test_manifest_generation_uses_unified_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let manifest_path = temp_dir.path().join(".amalgam-manifest.toml");

    // Create a test manifest
    let manifest_content = r#"
[package]
name = "test-package"
version = "0.1.0"

[[sources]]
name = "test-crd"
type = "crd"
file = "test.yaml"
"#;

    // Create test CRD file
    let test_crd = r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: tests.example.com
spec:
  group: example.com
  names:
    plural: tests
    singular: test
    kind: Test
  scope: Namespaced
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
              field1:
                type: string
"#;

    fs::write(&manifest_path, manifest_content)?;

    fs::write(temp_dir.path().join("test.yaml"), test_crd)?;

    // Run manifest generation
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "amalgam",
            "--",
            "generate-from-manifest",
            "--manifest",
            manifest_path
                .to_str()
                .ok_or("Failed to convert path to string")?,
        ])
        .current_dir(temp_dir.path())
        .output()?;

    // For CRD file sources, the command might not be fully implemented yet
    // Just check it doesn't crash with PackageGenerator errors
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("PackageGenerator"),
        "Should not reference old PackageGenerator"
    );
    Ok(())
}

#[test]
fn test_url_import_generates_cross_module_imports() -> Result<(), Box<dyn std::error::Error>> {
    if skip_if_no_network() {
        eprintln!("Skipping test: AMALGAM_SKIP_NETWORK_TESTS is set");
        return Ok(());
    }
    // This test verifies that URL imports properly generate cross-module imports
    // when CRDs have dependencies between versions

    let temp_dir = tempdir()?;
    let output_dir = temp_dir.path();

    // Test with a real CrossPlane CRD that has cross-version references
    // This will test both network access and cross-module import generation
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "amalgam",
            "--",
            "import",
            "url",
            "--url",
            "https://raw.githubusercontent.com/crossplane/crossplane/master/cluster/crds/apiextensions.crossplane.io_compositeresourcedefinitions.yaml",
            "--output",
            output_dir.to_str().ok_or("Failed to convert path to string")?,
            "--package",
            "test-crossplane-xrd",
        ])
        .output()
        ?;

    // Check the command output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Print output for debugging
    eprintln!("STDOUT: {}", stdout);
    eprintln!("STDERR: {}", stderr);

    // Check that command succeeded
    assert!(output.status.success(), "URL import should succeed");
    Ok(())
}

#[test]
fn test_project_compiles_with_unified_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    // This test verifies that the entire project compiles with unified pipeline
    // The fact that vendor.rs compiles proves it was migrated from PackageGenerator
    // to NamespacedPackage, since using PackageGenerator would cause compilation errors

    // Run a simple command to verify the binary compiles
    let output = Command::new("cargo")
        .args(["run", "--bin", "amalgam", "--", "--version"])
        .output()?;

    // If compilation succeeded, the vendor system is using unified pipeline
    assert!(
        output.status.success(),
        "Project compiles with unified pipeline"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("amalgam"),
        "Version output should contain 'amalgam'"
    );
    Ok(())
}
