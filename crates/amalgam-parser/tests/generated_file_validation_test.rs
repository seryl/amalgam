//! Tests that validate actual generated .ncl files for correctness
//!
//! These tests scan generated files and verify:
//! 1. Import bindings match their usage
//! 2. No dangling references
//! 3. Valid Nickel syntax
//! 4. Import paths resolve correctly

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[test]
fn test_all_generated_files_have_matching_bindings() {
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/pkgs");

    if !examples_dir.exists() {
        println!("Skipping test - examples directory doesn't exist");
        return;
    }

    let mut errors = Vec::new();

    for entry in WalkDir::new(&examples_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ncl")
            && !path.ends_with("mod.ncl")
            && !path.ends_with("Nickel-pkg.ncl")
        {
            if let Err(e) = validate_file_bindings(path) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        }
    }

    if !errors.is_empty() {
        panic!(
            "Found {} files with binding/usage mismatches:\n{}",
            errors.len(),
            errors.join("\n")
        );
    }
}

#[test]
fn test_all_import_paths_resolve() {
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/pkgs");

    if !examples_dir.exists() {
        println!("Skipping test - examples directory doesn't exist");
        return;
    }

    let mut errors = Vec::new();

    for entry in WalkDir::new(&examples_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ncl") {
            if let Err(e) = validate_import_paths_resolve(path) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        }
    }

    if !errors.is_empty() {
        panic!(
            "Found {} files with broken import paths:\n{}",
            errors.len(),
            errors.join("\n")
        );
    }
}

#[test]
fn test_no_dangling_references_in_generated_files() {
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/pkgs");

    if !examples_dir.exists() {
        println!("Skipping test - examples directory doesn't exist");
        return;
    }

    let mut errors = Vec::new();

    for entry in WalkDir::new(&examples_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ncl")
            && !path.ends_with("mod.ncl")
        {
            if let Err(e) = validate_no_dangling_refs(path) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        }
    }

    if !errors.is_empty() {
        panic!(
            "Found {} files with dangling references:\n{}",
            errors.len(),
            errors.join("\n")
        );
    }
}

#[test]
fn test_generated_files_valid_nickel_syntax() {
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/pkgs");

    if !examples_dir.exists() {
        println!("Skipping test - examples directory doesn't exist");
        return;
    }

    let mut errors = Vec::new();

    for entry in WalkDir::new(&examples_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ncl") {
            if let Err(e) = validate_nickel_syntax(path) {
                errors.push(format!("{}: {}", path.display(), e));
            }
        }
    }

    if !errors.is_empty() {
        panic!(
            "Found {} files with invalid Nickel syntax:\n{}",
            errors.len(),
            errors.join("\n")
        );
    }
}

// Validation helper functions

fn validate_file_bindings(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Extract import bindings: "let <binding> = import ..."
    let mut bindings = HashMap::new();
    for line in content.lines() {
        if line.trim().starts_with("let ") && line.contains("import") {
            if let Some(binding_part) = line.split('=').next() {
                let binding = binding_part
                    .trim()
                    .strip_prefix("let ")
                    .unwrap_or("")
                    .trim();

                // Extract type name from import path
                if let Some(import_path) = line.split('"').nth(1) {
                    let type_name = import_path
                        .trim_end_matches(".ncl")
                        .split('/')
                        .last()
                        .unwrap_or("")
                        .to_string();

                    bindings.insert(type_name, binding.to_string());
                }
            }
        }
    }

    // Check for mismatches
    let mut mismatches = Vec::new();
    for (type_name, binding) in &bindings {
        // Look for usage in contracts: | TypeName |
        if content.contains(&format!("| {} ", type_name))
            || content.contains(&format!("| {}\n", type_name))
        {
            if binding != type_name {
                mismatches.push(format!(
                    "Binding '{}' doesn't match usage '{}'. Should be: let {} = import \"...\"",
                    binding, type_name, type_name
                ));
            }
        }
    }

    if !mismatches.is_empty() {
        return Err(mismatches.join("; "));
    }

    Ok(())
}

fn validate_import_paths_resolve(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let base_dir = path.parent().ok_or("No parent directory")?;

    for line in content.lines() {
        if line.contains("import") && line.contains('"') {
            if let Some(import_path) = line.split('"').nth(1) {
                // Resolve the path relative to the current file
                let resolved = base_dir.join(import_path);

                if !resolved.exists() {
                    return Err(format!(
                        "Import path '{}' does not resolve to existing file. \
                         Expected: {}",
                        import_path,
                        resolved.display()
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_no_dangling_refs(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Extract available types (imported or file's own type)
    let mut available = HashSet::new();

    // Add imported types
    for line in content.lines() {
        if line.trim().starts_with("let ") && line.contains("import") {
            if let Some(binding_part) = line.split('=').next() {
                let binding = binding_part
                    .trim()
                    .strip_prefix("let ")
                    .unwrap_or("")
                    .trim();
                available.insert(binding.to_string());
            }
        }
    }

    // Add the file's own type (from filename)
    if let Some(filename) = path.file_stem() {
        if let Some(name) = filename.to_str() {
            available.insert(name.to_string());
        }
    }

    // Add primitive types
    available.insert("String".to_string());
    available.insert("Number".to_string());
    available.insert("Bool".to_string());
    available.insert("Array".to_string());

    // Extract used types from contracts
    let mut used = HashSet::new();
    for line in content.lines() {
        // Match: | TypeName |
        if let Some(parts) = line.split('|').nth(1) {
            let type_ref = parts.trim().split_whitespace().next().unwrap_or("");
            if !type_ref.is_empty()
                && type_ref.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                && !type_ref.starts_with('{')
                && !type_ref.contains("doc")
            {
                used.insert(type_ref.to_string());
            }
        }
    }

    // Check for dangling
    let dangling: Vec<_> = used.difference(&available).collect();
    if !dangling.is_empty() {
        return Err(format!(
            "Dangling references (used but not imported): {:?}",
            dangling
        ));
    }

    Ok(())
}

fn validate_nickel_syntax(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Basic syntax checks
    let open_braces = content.matches('{').count();
    let close_braces = content.matches('}').count();

    if open_braces != close_braces {
        return Err(format!(
            "Unbalanced braces: {} open, {} close",
            open_braces, close_braces
        ));
    }

    // Check import statements have proper syntax
    for (i, line) in content.lines().enumerate() {
        if line.contains("import") && !line.contains('#') {
            // Should have exactly 2 quotes
            let quote_count = line.matches('"').count();
            if quote_count != 2 {
                return Err(format!(
                    "Line {}: Import should have exactly 2 quotes: {}",
                    i + 1,
                    line
                ));
            }

            // Should have 'let' and '=' and 'in'
            if line.contains("let ") && !content[..content.find(line).unwrap() + line.len() + 100.min(content.len() - content.find(line).unwrap())].contains(" in") {
                return Err(format!(
                    "Line {}: Import with 'let' should have corresponding 'in': {}",
                    i + 1,
                    line
                ));
            }
        }
    }

    Ok(())
}

#[test]
fn test_specific_crossplane_composition_bindings() {
    // Specific regression test for the ObjectMeta binding issue
    let composition_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/pkgs/apiextensions_crossplane_io/v1/Composition.ncl");

    if !composition_file.exists() {
        println!("Skipping - Composition.ncl not generated yet");
        return;
    }

    let content = fs::read_to_string(&composition_file).unwrap();

    // Should have: let ObjectMeta = import "..."
    // NOT: let objectMeta = import "..."
    let has_correct_binding = content.contains("let ObjectMeta = import");
    let has_wrong_binding = content.contains("let objectMeta = import");

    if has_wrong_binding {
        panic!(
            "CRITICAL BUG: Composition.ncl has wrong binding case!\n\
             Found: let objectMeta = import ...\n\
             Should be: let ObjectMeta = import ...\n\
             \n\
             The usage is '| ObjectMeta |' (PascalCase) but binding is 'objectMeta' (camelCase).\n\
             This will cause Nickel runtime errors!"
        );
    }

    assert!(
        has_correct_binding,
        "Composition.ncl should have 'let ObjectMeta = import' binding"
    );
}
