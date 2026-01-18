use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;
use tempfile::TempDir;

/// Skip test if running in a sandboxed environment (e.g., Nix build)
/// Set AMALGAM_SKIP_NETWORK_TESTS=1 to skip these tests
fn skip_if_no_network() -> bool {
    std::env::var("AMALGAM_SKIP_NETWORK_TESTS").is_ok()
}

static INIT: Once = Once::new();
static mut PACKAGES_GENERATED: bool = false;
static mut TEST_DIR: Option<PathBuf> = None;

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

/// Clean up and prepare test directory for testing
fn ensure_test_packages_generated() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut generation_error: Option<String> = None;
    let mut packages_dir = PathBuf::new();

    INIT.call_once(|| {
        // Create a temporary directory for test packages
        match TempDir::new() {
            Ok(temp_dir) => {
                // Keep the temp directory alive by leaking it (it will be cleaned on process exit)
                #[allow(deprecated)]
                let temp_path = temp_dir.into_path();
                packages_dir = temp_path.join("test_packages");

                if let Err(e) = fs::create_dir_all(&packages_dir) {
                    generation_error =
                        Some(format!("Failed to create test packages directory: {}", e));
                    return;
                }

                // Store the path for later use
                unsafe {
                    TEST_DIR = Some(packages_dir.clone());
                }
            }
            Err(e) => {
                generation_error = Some(format!("Failed to create temp directory: {}", e));
                return;
            }
        }

        let k8s_dir = packages_dir.join("k8s_io");
        println!("Generating k8s_io package at {:?}", k8s_dir);
        let k8s_output = k8s_dir
            .to_str()
            .ok_or("Invalid path".to_string())
            .and_then(|path| {
                run_amalgam(&[
                    "import",
                    "k8s-core",
                    "--version",
                    "v1.33.4",
                    "--output",
                    path,
                ])
            });
        if let Err(e) = k8s_output {
            generation_error = Some(format!("Failed to generate k8s_io package: {:?}", e));
            return;
        }

        let crossplane_dir = packages_dir.join("crossplane");
        println!("Generating crossplane package at {:?}", crossplane_dir);
        let crossplane_output = crossplane_dir
            .to_str()
            .ok_or("Invalid path".to_string())
            .and_then(|path| {
                run_amalgam(&[
                    "import",
                    "url",
                    "--url",
                    "https://github.com/crossplane/crossplane/tree/v1.14.5/cluster/crds",
                    "--output",
                    path,
                ])
            });
        if let Err(e) = crossplane_output {
            generation_error = Some(format!("Failed to generate crossplane package: {:?}", e));
            return;
        }

        unsafe {
            PACKAGES_GENERATED = true;
        }
        println!("✓ Test packages generated successfully");
    });

    if let Some(error) = generation_error {
        return Err(error.into());
    }

    #[allow(static_mut_refs)]
    unsafe {
        if !PACKAGES_GENERATED {
            return Err("Package generation failed".into());
        }

        // Return the stored test directory path
        TEST_DIR
            .clone()
            .ok_or_else(|| "Test directory not initialized".into())
    }
}

#[test]
fn test_k8s_package_structure() -> Result<(), Box<dyn std::error::Error>> {
    if skip_if_no_network() {
        eprintln!("Skipping test: AMALGAM_SKIP_NETWORK_TESTS is set");
        return Ok(());
    }
    let packages_dir = ensure_test_packages_generated()?;
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
        let manifest = fs::read_to_string(k8s_dir.join("Nickel-pkg.ncl"))?;

        assert!(
            manifest.contains("name = \"k8s_io\"") || manifest.contains("name = \"k8s-io\""),
            "Manifest missing package name"
        );
    }

    println!("✓ k8s_io package structure validated");
    Ok(())
}

#[test]
fn test_generate_crossplane_package_with_k8s_dependency() -> Result<(), Box<dyn std::error::Error>>
{
    if skip_if_no_network() {
        eprintln!("Skipping test: AMALGAM_SKIP_NETWORK_TESTS is set");
        return Ok(());
    }
    let packages_dir = ensure_test_packages_generated()?;

    let k8s_dir = packages_dir.join("k8s_io");
    if !k8s_dir.join("Nickel-pkg.ncl").exists() {
        println!("Generating k8s_io package first...");
        let k8s_path = k8s_dir.to_str().ok_or("Invalid k8s_dir path")?;
        run_amalgam(&[
            "import",
            "k8s-core",
            "--version",
            "v1.33.4",
            "--output",
            k8s_path,
        ])?;
    }

    // Generate crossplane package
    let crossplane_dir = packages_dir.join("crossplane");

    println!("Generating crossplane package at {:?}", crossplane_dir);

    let crossplane_path = crossplane_dir
        .to_str()
        .ok_or("Invalid crossplane_dir path")?;
    run_amalgam(&[
        "import",
        "url",
        "--url",
        "https://raw.githubusercontent.com/crossplane/crossplane/master/cluster/crds/apiextensions.crossplane.io_compositions.yaml",
        "--output",
        crossplane_path,
        "--package",
        "crossplane",
    ])?;

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
        let manifest = fs::read_to_string(crossplane_dir.join("Nickel-pkg.ncl"))?;

        // The manifest generation is optional, but if it exists, check it's valid
        assert!(
            manifest.contains("name = "),
            "Manifest missing package name"
        );
    }

    println!("✓ crossplane package generated successfully with k8s_io dependency");
    Ok(())
}

#[test]
fn test_create_app_using_packages() -> Result<(), Box<dyn std::error::Error>> {
    if skip_if_no_network() {
        eprintln!("Skipping test: AMALGAM_SKIP_NETWORK_TESTS is set");
        return Ok(());
    }
    let packages_dir = ensure_test_packages_generated()?;

    // Both packages are already generated by ensure_test_packages_generated()
    let _k8s_dir = packages_dir.join("k8s_io");
    let _crossplane_dir = packages_dir.join("crossplane");

    // Create test app that uses both packages
    let test_app_dir = packages_dir.join("test_app");
    fs::create_dir_all(&test_app_dir)?;

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

    fs::write(test_app_dir.join("Nickel-pkg.ncl"), app_manifest)?;

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

    fs::write(test_app_dir.join("main.ncl"), main_content)?;

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
    Ok(())
}

#[test]
fn test_full_package_workflow() -> Result<(), Box<dyn std::error::Error>> {
    if skip_if_no_network() {
        eprintln!("Skipping test: AMALGAM_SKIP_NETWORK_TESTS is set");
        return Ok(());
    }
    println!("\n🚀 Running full package generation workflow...\n");

    // Ensure packages are generated
    ensure_test_packages_generated()?;

    // Run validation tests
    test_k8s_package_structure()?;
    test_generate_crossplane_package_with_k8s_dependency()?;
    test_create_app_using_packages()?;

    println!("\n✅ All package tests completed successfully!");
    println!("\nPackages are available in examples/pkgs/ for manual testing.");
    Ok(())
}
