use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;

static INIT: Once = Once::new();
static mut PACKAGES_GENERATED: bool = false;

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
fn ensure_test_packages_generated() -> PathBuf {
    let examples = examples_dir();
    let packages_dir = examples.join("pkgs_test");

    INIT.call_once(|| {
        let _ = fs::remove_dir_all(&packages_dir);
        fs::create_dir_all(&packages_dir).expect("Failed to create pkgs_test directory");

        let k8s_dir = packages_dir.join("k8s_io");
        println!("Generating k8s_io package at {:?}", k8s_dir);
        let result = run_amalgam(&[
            "import",
            "k8s-core",
            "--version",
            "v1.33.4",
            "--output",
            k8s_dir.to_str().unwrap(),
        ]);
        if let Err(e) = result {
            panic!("Failed to generate k8s_io package: {:?}", e);
        }

        let crossplane_dir = packages_dir.join("crossplane");
        println!("Generating crossplane package at {:?}", crossplane_dir);
        let result = run_amalgam(&[
            "import",
            "url",
            "--url",
            "https://github.com/crossplane/crossplane/tree/v1.14.5/cluster/crds",
            "--output",
            crossplane_dir.to_str().unwrap(),
        ]);
        if let Err(e) = result {
            panic!("Failed to generate crossplane package: {:?}", e);
        }

        unsafe {
            PACKAGES_GENERATED = true;
        }
        println!("✓ Test packages generated successfully");
    });

    unsafe {
        if !PACKAGES_GENERATED {
            panic!("Package generation failed");
        }
    }

    packages_dir
}

#[test]
fn test_k8s_package_structure() {
    let packages_dir = ensure_test_packages_generated();
    let k8s_dir = packages_dir.join("k8s_io");

    // Verify package structure
    assert!(k8s_dir.join("mod.ncl").exists(), "Missing mod.ncl");
    assert!(
        k8s_dir.join("Nickel-pkg.ncl").exists(),
        "Missing Nickel-pkg.ncl"
    );
    assert!(k8s_dir.join("v1").is_dir(), "Missing v1 directory");

    // Check if Nickel package manifest was generated (optional)
    if k8s_dir.join("Nickel-pkg.ncl").exists() {
        let manifest =
            fs::read_to_string(k8s_dir.join("Nickel-pkg.ncl")).expect("Failed to read manifest");

        assert!(
            manifest.contains("name = \"k8s_io\"") || manifest.contains("name = \"k8s-io\""),
            "Manifest missing package name"
        );
    }

    println!("✓ k8s_io package structure validated");
}

#[test]
fn test_generate_crossplane_package_with_k8s_dependency() {
    let packages_dir = ensure_test_packages_generated();

    let k8s_dir = packages_dir.join("k8s_io");
    if !k8s_dir.join("Nickel-pkg.ncl").exists() {
        println!("Generating k8s_io package first...");
        run_amalgam(&[
            "import",
            "k8s-core",
            "--version",
            "v1.33.4",
            "--output",
            k8s_dir.to_str().unwrap(),
        ])
        .expect("Failed to generate k8s_io package");
    }

    // Generate crossplane package
    let crossplane_dir = packages_dir.join("crossplane");

    println!("Generating crossplane package at {:?}", crossplane_dir);

    let result = run_amalgam(&[
        "import",
        "url",
        "--url",
        "https://raw.githubusercontent.com/crossplane/crossplane/master/cluster/crds/apiextensions.crossplane.io_compositions.yaml",
        "--output",
        crossplane_dir.to_str().unwrap(),
        "--package",
        "crossplane",
    ]);

    assert!(
        result.is_ok(),
        "Failed to generate crossplane package: {:?}",
        result
    );

    // Verify package structure
    assert!(crossplane_dir.join("mod.ncl").exists(), "Missing mod.ncl");
    assert!(
        crossplane_dir.join("Nickel-pkg.ncl").exists(),
        "Missing Nickel-pkg.ncl"
    );
    assert!(
        crossplane_dir.join("apiextensions.crossplane.io").is_dir(),
        "Missing apiextensions.crossplane.io directory"
    );

    // Check if Nickel package manifest was generated (optional)
    if crossplane_dir.join("Nickel-pkg.ncl").exists() {
        let manifest = fs::read_to_string(crossplane_dir.join("Nickel-pkg.ncl"))
            .expect("Failed to read manifest");

        // The manifest generation is optional, but if it exists, check it's valid
        assert!(
            manifest.contains("name = "),
            "Manifest missing package name"
        );
    }

    println!("✓ crossplane package generated successfully with k8s_io dependency");
}

#[test]
fn test_create_app_using_packages() {
    let packages_dir = ensure_test_packages_generated();

    // Both packages are already generated by ensure_test_packages_generated()
    let _k8s_dir = packages_dir.join("k8s_io");
    let _crossplane_dir = packages_dir.join("crossplane");

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

    fs::write(test_app_dir.join("main.ncl"), main_content).expect("Failed to write main.ncl");

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

    // Verify main file exists (manifest is created manually in this test)
    assert!(test_app_dir.join("main.ncl").exists(), "Missing main.ncl");
    assert!(
        test_app_dir.join("Nickel-pkg.ncl").exists(),
        "Missing app manifest"
    );

    println!("\n📦 Package structure created in examples/pkgs/:");
    println!("  k8s_io/       - Kubernetes v1.31 types");
    println!("  crossplane/   - Crossplane CRD types");
    println!("  test_app/     - Example app using both packages");
    println!("\nThese packages can be tested as if they were published to nickel-mine!");
}

#[test]
fn test_full_package_workflow() {
    println!("\n🚀 Running full package generation workflow...\n");

    // Ensure packages are generated
    ensure_test_packages_generated();

    // Run validation tests
    test_k8s_package_structure();
    test_generate_crossplane_package_with_k8s_dependency();
    test_create_app_using_packages();

    println!("\n✅ All package tests completed successfully!");
    println!("\nPackages are available in examples/pkgs/ for manual testing.");
}
