//! Import validation test
//!
//! This test walks generated `.ncl` files and verifies that all import paths resolve
//! to existing files. This catches issues where code generation produces imports
//! to non-existent modules.

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Extract all import paths from Nickel source code
fn extract_imports(content: &str) -> Vec<String> {
    let re = Regex::new(r#"import\s+"([^"]+)""#).unwrap();
    re.captures_iter(content)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// Resolve an import path relative to the file containing the import
fn resolve_import_path(import_path: &str, source_file: &Path) -> PathBuf {
    let source_dir = source_file.parent().unwrap_or(Path::new("."));
    source_dir.join(import_path)
}

/// Walk a directory and find all .ncl files
fn find_ncl_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !dir.exists() {
        return files;
    }

    fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path, files);
                } else if path.extension().is_some_and(|ext| ext == "ncl") {
                    files.push(path);
                }
            }
        }
    }

    walk_dir(dir, &mut files);
    files
}

/// Validate all imports in a directory of generated .ncl files
fn validate_imports(dir: &Path) -> Result<(), Vec<String>> {
    let ncl_files = find_ncl_files(dir);
    let mut errors = Vec::new();
    let mut checked_imports: HashSet<(PathBuf, String)> = HashSet::new();

    for file in &ncl_files {
        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                errors.push(format!("Failed to read {}: {}", file.display(), e));
                continue;
            }
        };

        let imports = extract_imports(&content);
        for import_path in imports {
            // Skip if we've already checked this exact import from this file
            let key = (file.clone(), import_path.clone());
            if checked_imports.contains(&key) {
                continue;
            }
            checked_imports.insert(key);

            let resolved = resolve_import_path(&import_path, file);
            let canonical = match resolved.canonicalize() {
                Ok(p) => p,
                Err(_) => {
                    // Try to provide a helpful error message
                    let relative_file = file
                        .strip_prefix(dir)
                        .unwrap_or(file)
                        .display()
                        .to_string();
                    errors.push(format!(
                        "Broken import in {}: \"{}\" -> {} (file not found)",
                        relative_file,
                        import_path,
                        resolved.display()
                    ));
                    continue;
                }
            };

            if !canonical.exists() {
                let relative_file = file
                    .strip_prefix(dir)
                    .unwrap_or(file)
                    .display()
                    .to_string();
                errors.push(format!(
                    "Broken import in {}: \"{}\" -> {} (file not found)",
                    relative_file,
                    import_path,
                    resolved.display()
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[test]
fn test_extract_imports() {
    let content = r#"
let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let core = import "../core/v1.ncl" in

MyType = {
    field | String
}
"#;

    let imports = extract_imports(content);
    assert_eq!(imports.len(), 2);
    assert!(imports.contains(&"../../apimachinery.pkg.apis/meta/v1/mod.ncl".to_string()));
    assert!(imports.contains(&"../core/v1.ncl".to_string()));
}

#[test]
fn test_extract_imports_with_various_formats() {
    // Test different import formats that might appear in Nickel files
    let content = r#"
let a = import "simple.ncl" in
let b = import "./relative.ncl" in
let c = import "../parent/file.ncl" in
let d = import "../../grandparent/deep/path.ncl" in
"#;

    let imports = extract_imports(content);
    assert_eq!(imports.len(), 4);
    assert!(imports.contains(&"simple.ncl".to_string()));
    assert!(imports.contains(&"./relative.ncl".to_string()));
    assert!(imports.contains(&"../parent/file.ncl".to_string()));
    assert!(imports.contains(&"../../grandparent/deep/path.ncl".to_string()));
}

#[test]
#[ignore] // Run with --ignored to validate examples directory
fn test_examples_imports_resolve() {
    // This test validates that all imports in the examples/pkgs directory resolve
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples")
        .join("pkgs");

    if !examples_dir.exists() {
        println!(
            "Skipping test: examples directory not found at {}",
            examples_dir.display()
        );
        return;
    }

    match validate_imports(&examples_dir) {
        Ok(()) => println!("All imports in examples/pkgs resolve successfully"),
        Err(errors) => {
            eprintln!("Found {} broken imports:", errors.len());
            for error in &errors {
                eprintln!("  - {}", error);
            }
            panic!(
                "Import validation failed with {} errors. See above for details.",
                errors.len()
            );
        }
    }
}

/// Helper function that can be called from other tests or CI
pub fn validate_generated_imports(dir: &Path) -> Result<(), String> {
    match validate_imports(dir) {
        Ok(()) => Ok(()),
        Err(errors) => {
            let mut msg = format!("Found {} broken imports:\n", errors.len());
            for error in errors {
                msg.push_str(&format!("  - {}\n", error));
            }
            Err(msg)
        }
    }
}
