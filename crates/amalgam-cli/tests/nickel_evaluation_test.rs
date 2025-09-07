use std::process::Command;
use std::path::PathBuf;

#[test]
#[ignore] // TODO: Update for new package structure where all types are in one file per module
fn test_generated_k8s_packages_evaluate() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the generated k8s packages
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("Failed to get parent directory")?
        .parent()
        .ok_or("Failed to get workspace root")?
        .join("examples");
    
    // Test that the k8s Pod type evaluates correctly
    let test_file = examples_dir.join("fixtures").join("nickel").join("test_k8s_pod.ncl");
    
    let output = Command::new("nickel")
        .arg("export")
        .arg("--format")
        .arg("json")
        .arg(&test_file)
        .current_dir(&examples_dir)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Nickel evaluation failed: {}", stderr).into());
    }
    
    // Basic check that output contains expected structure
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#""apiVersion": "v1""#), "Output should contain apiVersion");
    assert!(stdout.contains(r#""kind": "Pod""#), "Output should contain Pod kind");
    assert!(stdout.contains(r#""test-pod""#), "Output should contain test-pod name");
    
    Ok(())
}

#[test]
#[ignore] // TODO: Update for new package structure where all types are in one file per module
fn test_cross_package_imports() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the generated packages
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("Failed to get parent directory")?
        .parent()
        .ok_or("Failed to get workspace root")?
        .join("examples");
    
    // Test that crossplane packages can import k8s types
    let test_file = examples_dir.join("fixtures").join("nickel").join("test_crossplane_with_k8s.ncl");
    
    let output = Command::new("nickel")
        .arg("export")
        .arg("--format")
        .arg("json")
        .arg(&test_file)
        .current_dir(&examples_dir)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Nickel evaluation failed: {}", stderr).into());
    }
    
    // Check that the composition includes k8s metadata
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#""apiVersion": "apiextensions.crossplane.io/v1""#));
    assert!(stdout.contains(r#""kind": "Composition""#));
    
    Ok(())
}

#[test]
fn test_import_naming_conventions() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the test fixtures
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("Failed to get parent directory")?
        .parent()
        .ok_or("Failed to get workspace root")?
        .join("examples");
    
    // Test file that explicitly tests our naming conventions
    let test_file = examples_dir.join("fixtures").join("nickel").join("test_naming_conventions.ncl");
    
    let output = Command::new("nickel")
        .arg("export")
        .arg("--format")
        .arg("json")
        .arg(&test_file)
        .current_dir(&examples_dir)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Nickel evaluation failed: {}", stderr).into());
    }
    
    // The test should pass if our naming conventions work
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("naming_test_passed"));
    
    Ok(())
}