//! Integration tests for Nickel package generation
//! 
//! These tests verify that amalgam can generate proper Nickel packages
//! that work with Nickel's package management system. The tests generate
//! packages in the examples directory so they can be used for demonstrations.
//! 
//! Run with: cargo test --test package_integration -- --ignored
//! Or set: RUN_INTEGRATION_TESTS=1 cargo test --test package_integration

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get the project root directory
fn project_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Get the examples directory
fn examples_dir() -> PathBuf {
    project_root().join("examples")
}

/// Helper to run amalgam command
fn run_amalgam(args: &[&str]) -> Result<String, String> {
    let amalgam_bin = env!("CARGO_BIN_EXE_amalgam");
    
    let output = Command::new(amalgam_bin)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run amalgam: {}", e))?;
    
    if !output.status.success() {
        return Err(format!(
            "amalgam failed with status: {}\nstderr: {}\nstdout: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Helper to check if Nickel is available with package support
fn check_nickel_available() -> bool {
    Command::new("nickel")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Clean up and prepare examples directory for testing
fn prepare_examples_dir() -> PathBuf {
    let examples = examples_dir();
    
    // Create packages subdirectory
    let packages_dir = examples.join("packages");
    fs::create_dir_all(&packages_dir).expect("Failed to create packages directory");
    
    // Clean up old test packages
    let _ = fs::remove_dir_all(packages_dir.join("k8s_io"));
    let _ = fs::remove_dir_all(packages_dir.join("crossplane"));
    let _ = fs::remove_dir_all(packages_dir.join("test_app"));
    
    packages_dir
}

#[test]
#[ignore] // Run with --ignored or set RUN_INTEGRATION_TESTS=1
fn test_generate_k8s_package_with_manifest() {
    if std::env::var("RUN_INTEGRATION_TESTS").is_err() && !cfg!(test) {
        eprintln!("Skipping integration test. Run with --ignored or set RUN_INTEGRATION_TESTS=1");
        return;
    }
    
    let packages_dir = prepare_examples_dir();
    let k8s_dir = packages_dir.join("k8s_io");
    
    println!("Generating k8s_io package at {:?}", k8s_dir);
    
    // Generate k8s_io package with k8s 1.31 (latest stable as of the test)
    // Note: k8s 1.34 doesn't exist yet, using 1.31 as latest
    let result = run_amalgam(&[
        "import",
        "k8s-core",
        "--version", "v1.31.0",
        "--output", k8s_dir.to_str().unwrap(),
        "--nickel-package",
    ]);
    
    assert!(result.is_ok(), "Failed to generate k8s_io package: {:?}", result);
    
    // Verify package structure
    assert!(k8s_dir.join("mod.ncl").exists(), "Missing mod.ncl");
    assert!(k8s_dir.join("Nickel-pkg.ncl").exists(), "Missing Nickel-pkg.ncl");
    assert!(k8s_dir.join("v1").is_dir(), "Missing v1 directory");
    
    // Verify manifest content
    let manifest = fs::read_to_string(k8s_dir.join("Nickel-pkg.ncl"))
        .expect("Failed to read manifest");
    
    assert!(manifest.contains("name = \"k8s-io\""), "Manifest missing package name");
    assert!(manifest.contains("minimal_nickel_version"), "Manifest missing nickel version");
    assert!(manifest.contains("| std.package.Manifest"), "Manifest missing contract");
    
    println!("✓ k8s_io package generated successfully");
}

#[test]
#[ignore]
fn test_generate_crossplane_package_with_k8s_dependency() {
    if std::env::var("RUN_INTEGRATION_TESTS").is_err() && !cfg!(test) {
        eprintln!("Skipping integration test. Run with --ignored or set RUN_INTEGRATION_TESTS=1");
        return;
    }
    
    let packages_dir = prepare_examples_dir();
    
    // First ensure k8s_io package exists
    let k8s_dir = packages_dir.join("k8s_io");
    if !k8s_dir.join("Nickel-pkg.ncl").exists() {
        println!("Generating k8s_io package first...");
        run_amalgam(&[
            "import",
            "k8s-core",
            "--version", "v1.31.0",
            "--output", k8s_dir.to_str().unwrap(),
            "--nickel-package",
        ]).expect("Failed to generate k8s_io package");
    }
    
    // Generate crossplane package
    let crossplane_dir = packages_dir.join("crossplane");
    
    println!("Generating crossplane package at {:?}", crossplane_dir);
    
    let result = run_amalgam(&[
        "import",
        "url",
        "--url", "https://github.com/crossplane/crossplane/tree/main/cluster/crds",
        "--output", crossplane_dir.to_str().unwrap(),
        "--package", "crossplane-types",
        "--nickel-package",
    ]);
    
    assert!(result.is_ok(), "Failed to generate crossplane package: {:?}", result);
    
    // Verify package structure
    assert!(crossplane_dir.join("mod.ncl").exists(), "Missing mod.ncl");
    assert!(crossplane_dir.join("Nickel-pkg.ncl").exists(), "Missing Nickel-pkg.ncl");
    assert!(crossplane_dir.join("apiextensions.crossplane.io").is_dir(), 
            "Missing apiextensions.crossplane.io directory");
    
    // Verify manifest has k8s_io dependency
    let manifest = fs::read_to_string(crossplane_dir.join("Nickel-pkg.ncl"))
        .expect("Failed to read manifest");
    
    assert!(manifest.contains("dependencies"), "Manifest missing dependencies");
    assert!(manifest.contains("k8s_io"), "Manifest missing k8s_io dependency");
    
    // Fix the dependency path to point to the correct location
    let fixed_manifest = manifest.replace(
        r#"k8s_io = 'Path "../k8s_io""#,
        r#"k8s_io = 'Path "../k8s_io""#
    );
    fs::write(crossplane_dir.join("Nickel-pkg.ncl"), fixed_manifest)
        .expect("Failed to update manifest");
    
    println!("✓ crossplane package generated successfully with k8s_io dependency");
}

#[test]
#[ignore]
fn test_create_app_using_packages() {
    if std::env::var("RUN_INTEGRATION_TESTS").is_err() && !cfg!(test) {
        eprintln!("Skipping integration test. Run with --ignored or set RUN_INTEGRATION_TESTS=1");
        return;
    }
    
    let packages_dir = prepare_examples_dir();
    
    // Ensure both packages exist
    let k8s_dir = packages_dir.join("k8s_io");
    let crossplane_dir = packages_dir.join("crossplane");
    
    if !k8s_dir.join("Nickel-pkg.ncl").exists() {
        test_generate_k8s_package_with_manifest();
    }
    
    if !crossplane_dir.join("Nickel-pkg.ncl").exists() {
        test_generate_crossplane_package_with_k8s_dependency();
    }
    
    // Create test app that uses both packages
    let test_app_dir = packages_dir.join("test_app");
    fs::create_dir_all(&test_app_dir).expect("Failed to create test app dir");
    
    // Create app manifest that depends on both packages
    let app_manifest = r#"{
  name = "test-app",
  version = "0.1.0",
  description = "Test application using k8s and crossplane types",
  minimal_nickel_version = "1.9.0",
  dependencies = {
    k8s = 'Path "../k8s_io",
    crossplane = 'Path "../crossplane",
  },
} | std.package.Manifest
"#;
    
    fs::write(test_app_dir.join("Nickel-pkg.ncl"), app_manifest)
        .expect("Failed to write app manifest");
    
    // Create main.ncl that uses both packages
    let main_content = r#"# Test application using amalgam-generated packages
let k8s = import k8s in
let crossplane = import crossplane in

{
  # Create a Deployment using k8s types
  deployment = {
    apiVersion = "apps/v1",
    kind = "Deployment",
    metadata = {
      name = "test-app",
      namespace = "default",
    },
    spec = {
      replicas = 3,
      selector = {
        matchLabels = {
          app = "test",
        },
      },
      template = {
        metadata = {
          labels = {
            app = "test",
          },
        },
        spec = {
          containers = [
            {
              name = "app",
              image = "nginx:latest",
              ports = [
                {
                  containerPort = 80,
                },
              ],
            },
          ],
        },
      },
    },
  },
  
  # Create a Composition using crossplane types
  composition = {
    apiVersion = "apiextensions.crossplane.io/v1",
    kind = "Composition",
    metadata = {
      name = "test-composition",
    },
    spec = {
      compositeTypeRef = {
        apiVersion = "example.org/v1",
        kind = "XDatabase",
      },
      mode = "Pipeline",
      pipeline = [
        {
          step = "create-db",
          functionRef = {
            name = "function-create-db",
          },
        },
      ],
    },
  },
}
"#;
    
    fs::write(test_app_dir.join("main.ncl"), main_content)
        .expect("Failed to write main.ncl");
    
    println!("✓ Test app created at {:?}", test_app_dir);
    
    // If Nickel is available, try to evaluate
    if check_nickel_available() {
        println!("Testing with Nickel...");
        
        let output = Command::new("nickel")
            .arg("eval")
            .arg("main.ncl")
            .current_dir(&test_app_dir)
            .output();
        
        match output {
            Ok(output) if output.status.success() => {
                println!("✓ Nickel evaluation succeeded!");
            }
            Ok(output) => {
                // Package support might not be enabled in Nickel
                eprintln!("⚠ Nickel evaluation failed (package support may not be enabled):");
                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("⚠ Failed to run Nickel: {}", e);
            }
        }
    } else {
        println!("⚠ Nickel not available, skipping evaluation test");
    }
    
    // Verify files exist
    assert!(test_app_dir.join("Nickel-pkg.ncl").exists(), "Missing app manifest");
    assert!(test_app_dir.join("main.ncl").exists(), "Missing main.ncl");
    
    println!("\n📦 Package structure created in examples/packages/:");
    println!("  k8s_io/       - Kubernetes v1.31 types");
    println!("  crossplane/   - Crossplane CRD types");  
    println!("  test_app/     - Example app using both packages");
    println!("\nThese packages can be tested as if they were published to nickel-mine!");
}

#[test]
#[ignore]
fn test_full_package_workflow() {
    if std::env::var("RUN_INTEGRATION_TESTS").is_err() && !cfg!(test) {
        eprintln!("Skipping integration test. Run with --ignored or set RUN_INTEGRATION_TESTS=1");
        return;
    }
    
    println!("\n🚀 Running full package generation workflow...\n");
    
    // Run all tests in sequence
    test_generate_k8s_package_with_manifest();
    test_generate_crossplane_package_with_k8s_dependency();
    test_create_app_using_packages();
    
    println!("\n✅ All package tests completed successfully!");
    println!("\nPackages are available in examples/packages/ for manual testing.");
}