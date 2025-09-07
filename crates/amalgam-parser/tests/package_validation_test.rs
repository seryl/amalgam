//! Tests to validate that generated packages have correct structure and naming
//!
//! These tests verify:
//! - Files use PascalCase naming
//! - Import statements use camelCase variables
//! - Import paths reference PascalCase files
//! - Type references use camelCase variables

use std::fs;
use std::path::Path;
use amalgam_core::naming::to_camel_case;

/// Validates that a generated package follows naming conventions
fn validate_package_structure(package_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Check that the package directory exists
    if !package_path.exists() {
        return Err(format!("Package path does not exist: {:?}", package_path).into());
    }

    // Find all .ncl files in the package
    let mut validation_errors = Vec::new();
    validate_directory(package_path, &mut validation_errors)?;

    if !validation_errors.is_empty() {
        return Err(format!(
            "Package validation failed with {} errors:\n{}",
            validation_errors.len(),
            validation_errors.join("\n")
        )
        .into());
    }

    Ok(())
}

/// Recursively validate all .ncl files in a directory
fn validate_directory(
    dir: &Path,
    errors: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            validate_directory(&path, errors)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("ncl") {
            if let Err(e) = validate_nickel_file(&path) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        }
    }
    Ok(())
}

/// Validate a single Nickel file
fn validate_nickel_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid file name")?;

    // Skip mod.ncl files
    if file_name == "mod" {
        return Ok(());
    }

    // Check file name is PascalCase (unless it's a special file)
    if !is_pascal_case(file_name) && file_name != "intorstring" {
        return Err(format!("File name '{}' is not PascalCase", file_name).into());
    }

    // Check imports
    for line in content.lines() {
        if line.trim().starts_with("let ") && line.contains(" = import ") {
            validate_import_line(line)?;
        }

        // Check type references in arrays
        if line.contains("Array ") {
            validate_array_reference(line)?;
        }
    }

    Ok(())
}

/// Validate an import line follows conventions
fn validate_import_line(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse: let variableName = import "./FileName.ncl" in
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    if parts.len() < 5 {
        return Ok(()); // Skip malformed lines
    }

    let var_name = parts[1];
    let import_path = parts[4].trim_matches('"');

    // Check variable name is camelCase
    if !is_camel_case(var_name) {
        return Err(format!(
            "Import variable '{}' should be camelCase",
            var_name
        )
        .into());
    }

    // Extract filename from import path
    if let Some(file_name) = import_path.split('/').last() {
        if let Some(name) = file_name.strip_suffix(".ncl") {
            // Check imported file name is PascalCase
            if !is_pascal_case(name) && name != "intorstring" && name != "mod" {
                return Err(format!(
                    "Imported file '{}' should be PascalCase",
                    name
                )
                .into());
            }

            // Check that variable name matches file name (camelCase version)
            let expected_var = to_camel_case(name);
            if var_name != expected_var {
                return Err(format!(
                    "Import variable '{}' doesn't match expected '{}' for file '{}'",
                    var_name, expected_var, name
                )
                .into());
            }
        }
    }

    Ok(())
}

/// Validate array type references
fn validate_array_reference(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Look for patterns like "Array managedFieldsEntry"
    if let Some(idx) = line.find("Array ") {
        let after_array = &line[idx + 6..];
        if let Some(type_ref) = after_array.split_whitespace().next() {
            // Skip built-in types
            if !["String", "Number", "Bool"].contains(&type_ref) {
                // Type reference should be camelCase (variable name)
                if !is_camel_case(type_ref) && !type_ref.contains('{') {
                    return Err(format!(
                        "Array type reference '{}' should be camelCase",
                        type_ref
                    )
                    .into());
                }
            }
        }
    }
    Ok(())
}

/// Check if a string is PascalCase
fn is_pascal_case(s: &str) -> bool {
    !s.is_empty() && s.chars().next().map_or(false, |c| c.is_uppercase())
}

/// Check if a string is camelCase
fn is_camel_case(s: &str) -> bool {
    !s.is_empty() && s.chars().next().map_or(false, |c| c.is_lowercase())
}


#[test]
fn test_k8s_package_structure() -> Result<(), Box<dyn std::error::Error>> {
    let k8s_path = Path::new("examples/pkgs/k8s_io");
    
    // Skip if examples not generated
    if !k8s_path.exists() {
        eprintln!("Skipping test - k8s_io package not found. Run regenerate-examples first.");
        return Ok(());
    }

    validate_package_structure(k8s_path)?;
    Ok(())
}

#[test]
fn test_crossplane_package_structure() -> Result<(), Box<dyn std::error::Error>> {
    let crossplane_path = Path::new("examples/pkgs/crossplane");
    
    // Skip if examples not generated
    if !crossplane_path.exists() {
        eprintln!("Skipping test - crossplane package not found. Run regenerate-examples first.");
        return Ok(());
    }

    validate_package_structure(crossplane_path)?;
    Ok(())
}

#[test]
fn test_objectmeta_imports() -> Result<(), Box<dyn std::error::Error>> {
    let objectmeta_path = Path::new("examples/pkgs/k8s_io/v1/ObjectMeta.ncl");
    
    // Skip if file doesn't exist
    if !objectmeta_path.exists() {
        eprintln!("Skipping test - ObjectMeta.ncl not found. Run regenerate-examples first.");
        return Ok(());
    }

    let content = fs::read_to_string(objectmeta_path)?;
    
    // Check for expected imports
    assert!(
        content.contains("let managedFieldsEntry = import"),
        "ObjectMeta should import managedFieldsEntry"
    );
    assert!(
        content.contains("let ownerReference = import"),
        "ObjectMeta should import ownerReference"
    );
    
    // Check that references use camelCase variables
    assert!(
        content.contains("Array managedFieldsEntry"),
        "Should reference managedFieldsEntry with camelCase"
    );
    assert!(
        content.contains("Array ownerReference"),
        "Should reference ownerReference with camelCase"
    );
    
    // Check that problematic reference is fixed
    assert!(
        !content.contains("managedfieldsentry.ManagedFieldsEntry"),
        "Should not contain problematic lowercase module reference"
    );
    
    Ok(())
}

#[test]
fn test_import_path_conventions() -> Result<(), Box<dyn std::error::Error>> {
    // Test our helper functions
    assert!(is_pascal_case("ManagedFieldsEntry"));
    assert!(!is_pascal_case("managedFieldsEntry"));
    assert!(is_camel_case("managedFieldsEntry"));
    assert!(!is_camel_case("ManagedFieldsEntry"));
    
    assert_eq!(to_camel_case("ManagedFieldsEntry"), "managedFieldsEntry");
    assert_eq!(to_camel_case("Pod"), "pod");
    
    // Test import line validation
    let valid_import = r#"let managedFieldsEntry = import "./ManagedFieldsEntry.ncl" in"#;
    validate_import_line(valid_import)?;
    
    let invalid_var = r#"let ManagedFieldsEntry = import "./ManagedFieldsEntry.ncl" in"#;
    assert!(validate_import_line(invalid_var).is_err());
    
    let invalid_file = r#"let managedFieldsEntry = import "./managedfieldsentry.ncl" in"#;
    assert!(validate_import_line(invalid_file).is_err());
    
    Ok(())
}