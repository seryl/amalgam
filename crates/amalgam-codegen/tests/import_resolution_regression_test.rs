//! Regression tests for import resolution bugs
//!
//! These tests capture specific bugs we've encountered to prevent regressions.
//! Each test should document the original issue it's preventing.

use amalgam_codegen::resolver::{ResolutionContext, TypeResolver};
use amalgam_core::ir::{Import, Metadata, Module};
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Regression test for crossplane type resolution
///
/// Original issue: When resolving "apiextensions.crossplane.io/v1/Composition"
/// with an import for "../../apiextensions.crossplane.io/v1/composition.ncl",
/// the resolver was not matching because the type extraction logic was broken.
#[test]
fn test_crossplane_composition_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test".to_string(),
        imports: vec![Import {
            path: "../../apiextensions.crossplane.io/v1/composition.ncl".to_string(),
            alias: Some("composition".to_string()),
            items: vec!["Composition".to_string()],
        }],
        types: vec![],
        constants: vec![],
        metadata: Metadata {
            source_language: None,
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };

    let context = ResolutionContext::default();

    // This should resolve to "composition.Composition"
    let resolved = resolver.resolve(
        "apiextensions.crossplane.io/v1/Composition",
        &module,
        &context,
    );

    assert_eq!(
        resolved, "composition.Composition",
        "Crossplane Composition type should be resolved with the import alias"
    );
    Ok(())
}

/// Regression test for k8s apimachinery type resolution with module imports
///
/// Original issue: Module imports (mod.ncl) were not matching correctly
/// for k8s types when using short names like "ObjectMeta"
#[test]
fn test_k8s_module_import_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test".to_string(),
        imports: vec![Import {
            path: "../../k8s.io/apimachinery/v1/mod.ncl".to_string(),
            alias: Some("k8s_v1".to_string()),
            items: vec![],
        }],
        types: vec![],
        constants: vec![],
        metadata: Metadata {
            source_language: None,
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };

    let context = ResolutionContext::default();

    // Test short name resolution
    let resolved = resolver.resolve("ObjectMeta", &module, &context);
    assert_eq!(
        resolved, "k8s_v1.ObjectMeta",
        "Short name ObjectMeta should resolve to k8s_v1.ObjectMeta"
    );

    // Test full name resolution
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context,
    );
    assert_eq!(
        resolved, "k8s_v1.ObjectMeta",
        "Full k8s path should resolve to k8s_v1.ObjectMeta"
    );
    Ok(())
}

/// Regression test for multiple k8s imports with correct alias matching
///
/// Original issue: When multiple k8s type files were imported (e.g., objectmeta.ncl,
/// volume.ncl, resourcerequirements.ncl), all references were incorrectly using
/// the first import's alias instead of their specific aliases.
#[test]
fn test_multiple_k8s_type_file_imports() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test.io.v1.multiref".to_string(),
        imports: vec![
            Import {
                path: "../../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("objectmeta".to_string()),
                items: vec!["ObjectMeta".to_string()],
            },
            Import {
                path: "../../../k8s_io/v1/volume.ncl".to_string(),
                alias: Some("volume".to_string()),
                items: vec!["Volume".to_string()],
            },
            Import {
                path: "../../../k8s_io/v1/resourcerequirements.ncl".to_string(),
                alias: Some("resourcerequirements".to_string()),
                items: vec!["ResourceRequirements".to_string()],
            },
        ],
        types: vec![],
        constants: vec![],
        metadata: Metadata {
            source_language: None,
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };

    let context = ResolutionContext::default();

    // Each type should resolve to its specific import alias
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context,
    );
    assert_eq!(
        resolved, "objectmeta.ObjectMeta",
        "ObjectMeta should use the objectmeta import alias"
    );

    let resolved = resolver.resolve("io.k8s.api.core.v1.Volume", &module, &context);
    assert_eq!(
        resolved, "volume.Volume",
        "Volume should use the volume import alias"
    );

    let resolved = resolver.resolve("io.k8s.api.core.v1.ResourceRequirements", &module, &context);
    assert_eq!(
        resolved, "resourcerequirements.ResourceRequirements",
        "ResourceRequirements should use the resourcerequirements import alias"
    );
    Ok(())
}

/// Regression test for package directory structure and import paths
///
/// Original issue: Generated packages had incorrect directory structure where
/// k8s-core import put files under examples/pkgs/k8s_io/ instead of proper
/// package structure, and import paths used ../../../k8s_io/v1/... instead
/// of ../v1/... within the same package.
#[test]
fn test_package_structure_and_import_paths() -> Result<(), Box<dyn std::error::Error>> {
    let examples_dir = Path::new("../../../examples/pkgs");

    // Skip test if examples directory doesn't exist (CI environments)
    if !examples_dir.exists() {
        println!("Skipping package structure test - examples directory not found");
        return;
    }

    let mut validation_errors = Vec::new();

    // Test k8s_io package structure and imports
    validate_k8s_io_package(&examples_dir.join("k8s_io"), &mut validation_errors);

    // Test crossplane package structure and imports
    validate_crossplane_package(&examples_dir.join("crossplane"), &mut validation_errors);

    // Test general import path patterns across all packages
    validate_general_import_patterns(examples_dir, &mut validation_errors);

    if !validation_errors.is_empty() {
        return Err(format!(
            "Package structure regression test failed with {} errors:\n{}",
            validation_errors.len(),
            validation_errors.join("\n")
        ).into());
    }
    Ok(())
}

fn validate_k8s_io_package(k8s_io_dir: &Path, errors: &mut Vec<String>) {
    if !k8s_io_dir.exists() {
        errors.push("k8s_io package directory does not exist".to_string());
        return;
    }

    // Check required files exist
    let required_files = vec!["Nickel-pkg.ncl", "mod.ncl"];

    for file in required_files {
        if !k8s_io_dir.join(file).exists() {
            errors.push(format!("k8s_io package missing required file: {}", file));
        }
    }

    // Check version directories exist
    let version_dirs = vec!["v1", "v2"];
    for version in version_dirs {
        let version_dir = k8s_io_dir.join(version);
        if !version_dir.exists() {
            errors.push(format!(
                "k8s_io package missing version directory: {}",
                version
            ));
            continue;
        }

        // Check that version directories have mod.ncl
        if !version_dir.join("mod.ncl").exists() {
            errors.push(format!("k8s_io/{} missing mod.ncl", version));
        }
    }

    // Validate import paths within k8s_io package
    validate_package_imports(k8s_io_dir, "k8s_io", errors);
}

fn validate_crossplane_package(crossplane_dir: &Path, errors: &mut Vec<String>) {
    if !crossplane_dir.exists() {
        errors.push("crossplane package directory does not exist".to_string());
        return;
    }

    // Check required files exist
    let required_files = vec!["Nickel-pkg.ncl", "mod.ncl"];

    for file in required_files {
        if !crossplane_dir.join(file).exists() {
            errors.push(format!(
                "crossplane package missing required file: {}",
                file
            ));
        }
    }

    // Validate that crossplane manifest has k8s_io dependency
    if let Ok(manifest_content) = std::fs::read_to_string(crossplane_dir.join("Nickel-pkg.ncl")) {
        if !manifest_content.contains("k8s_io") || !manifest_content.contains("dependencies") {
            errors.push("crossplane manifest missing k8s_io dependency".to_string());
        }
    } else {
        errors.push("Failed to read crossplane Nickel-pkg.ncl".to_string());
    }

    // Validate import paths within crossplane package
    validate_package_imports(crossplane_dir, "crossplane", errors);
}

fn validate_package_imports(package_dir: &Path, package_name: &str, errors: &mut Vec<String>) {
    for entry in WalkDir::new(package_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                errors.push(format!("Failed to walk {}: {}", package_name, e));
                continue;
            }
        };

        if !entry.file_type().is_file()
            || !entry.path().extension().is_some_and(|ext| ext == "ncl")
        {
            continue;
        }

        let content = match std::fs::read_to_string(entry.path()) {
            Ok(content) => content,
            Err(e) => {
                errors.push(format!("Failed to read {}: {}", entry.path().display(), e));
                continue;
            }
        };

        validate_file_imports(&content, entry.path(), package_name, errors);
    }
}

fn validate_file_imports(
    content: &str,
    file_path: &Path,
    package_name: &str,
    errors: &mut Vec<String>,
) {
    let file_display = file_path.display().to_string();

    // Check for legacy import patterns that should not exist
    let forbidden_patterns = vec![
        (
            "../../../k8s_io/",
            "should use relative imports within package or proper cross-package imports",
        ),
        (
            "../../k8s_io/",
            "should use relative imports within package or proper cross-package imports",
        ),
        ("../../../../", "too many parent directory traversals"),
        (
            "../../../",
            "excessive parent directory traversals - check if import is correct",
        ),
    ];

    for (pattern, reason) in forbidden_patterns {
        if content.contains(pattern) {
            errors.push(format!(
                "{}: contains forbidden import pattern '{}' - {}",
                file_display, pattern, reason
            ));
        }
    }

    // Validate import paths are reasonable
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("let ") && line.contains("import \"") {
            if let Some(import_start) = line.find("import \"") {
                if let Some(import_end) = line[import_start + 8..].find("\"") {
                    let import_path = &line[import_start + 8..import_start + 8 + import_end];
                    validate_import_path(import_path, file_path, package_name, errors);
                }
            }
        }
    }
}

fn validate_import_path(
    import_path: &str,
    file_path: &Path,
    package_name: &str,
    errors: &mut Vec<String>,
) {
    let file_display = file_path.display().to_string();

    // Check for valid relative imports within the same package
    if import_path.starts_with("../") {
        let import_depth = import_path.matches("../").count();

        // Calculate expected depth based on file location
        let file_components: Vec<_> = file_path.components().collect();
        let pkg_index = file_components
            .iter()
            .position(|c| c.as_os_str().to_string_lossy() == package_name);

        if let Some(pkg_idx) = pkg_index {
            let file_depth = file_components.len() - pkg_idx - 2; // -1 for pkg dir, -1 for file itself

            // For imports within the same package, depth should be reasonable
            if import_depth > file_depth + 2 {
                errors.push(format!(
                    "{}: import '{}' has suspicious depth {} (file depth: {})",
                    file_display, import_path, import_depth, file_depth
                ));
            }
        }
    }

    // Check that imports end with .ncl
    if !import_path.ends_with(".ncl") {
        errors.push(format!(
            "{}: import '{}' should end with .ncl",
            file_display, import_path
        ));
    }

    // Check for common typos or invalid paths
    if import_path.contains("//") {
        errors.push(format!(
            "{}: import '{}' contains double slashes",
            file_display, import_path
        ));
    }

    if import_path.starts_with("/") {
        errors.push(format!(
            "{}: import '{}' uses absolute path - should be relative",
            file_display, import_path
        ));
    }
}

fn validate_general_import_patterns(examples_dir: &Path, errors: &mut Vec<String>) {
    let mut all_ncl_files = Vec::new();
    let mut import_relationships = Vec::new();

    // Collect all .ncl files and their imports
    for entry in WalkDir::new(examples_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                errors.push(format!("Failed to walk examples directory: {}", e));
                continue;
            }
        };

        if !entry.file_type().is_file()
            || !entry.path().extension().is_some_and(|ext| ext == "ncl")
        {
            continue;
        }

        all_ncl_files.push(entry.path().to_path_buf());

        let content = match std::fs::read_to_string(entry.path()) {
            Ok(content) => content,
            Err(_) => continue,
        };

        // Extract import statements
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("let ") && line.contains("import \"") {
                if let Some(import_start) = line.find("import \"") {
                    if let Some(import_end) = line[import_start + 8..].find("\"") {
                        let import_path = &line[import_start + 8..import_start + 8 + import_end];
                        import_relationships
                            .push((entry.path().to_path_buf(), import_path.to_string()));
                    }
                }
            }
        }
    }

    // Validate that imported files exist or are reasonable external imports
    for (importer, imported) in &import_relationships {
        if imported.starts_with("../") || !imported.contains("/") {
            // This is a local import, check if the file exists
            let parent = match importer.parent() {
                Some(p) => p,
                None => continue,
            };
            let import_path = parent.join(imported);
            if !import_path.exists() {
                errors.push(format!(
                    "{}: imports '{}' but file does not exist at {}",
                    importer.display(),
                    imported,
                    import_path.display()
                ));
            }
        }
    }

    // Check for circular dependencies (basic check)
    let mut checked_files = HashSet::new();
    let mut checking_stack = Vec::new();

    for file in &all_ncl_files {
        if !checked_files.contains(file) {
            check_circular_imports(
                file,
                &import_relationships,
                &mut checked_files,
                &mut checking_stack,
                errors,
            );
        }
    }
}

fn check_circular_imports(
    current_file: &Path,
    relationships: &[(PathBuf, String)],
    checked: &mut HashSet<PathBuf>,
    stack: &mut Vec<PathBuf>,
    errors: &mut Vec<String>,
) {
    let current_file_buf = current_file.to_path_buf();
    if stack.contains(&current_file_buf) {
        let cycle_start = match stack.iter().position(|f| f == &current_file_buf) {
            Some(pos) => pos,
            None => return,
        };
        let cycle: Vec<String> = stack[cycle_start..]
            .iter()
            .chain(std::iter::once(&current_file_buf))
            .map(|p| p.display().to_string())
            .collect();
        errors.push(format!("Circular import detected: {}", cycle.join(" -> ")));
        return;
    }

    if checked.contains(&current_file_buf) {
        return;
    }

    stack.push(current_file_buf.clone());

    // Find imports from current file
    for (importer, imported) in relationships {
        if importer == &current_file_buf && imported.starts_with("../") {
            let import_path = current_file.parent()?.join(imported);
            if let Ok(canonical_import) = import_path.canonicalize() {
                check_circular_imports(&canonical_import, relationships, checked, stack, errors);
            }
        }
    }

    stack.pop();
    checked.insert(current_file_buf);
}

/// Regression test for Nickel package manifest structure
///
/// Original issue: Generated packages were missing proper manifest structure
/// and didn't mention they were generated by Amalgam.
#[test]
fn test_package_manifest_structure() -> Result<(), Box<dyn std::error::Error>> {
    let examples_dir = Path::new("../../../examples/pkgs");

    if !examples_dir.exists() {
        println!("Skipping package manifest test - examples directory not found");
        return;
    }

    let mut validation_errors = Vec::new();

    // Check that all package directories have required structure
    for entry in std::fs::read_dir(examples_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let package_name = entry.file_name().to_string_lossy().to_string();
        let package_dir = entry.path();

        // Required files for every package
        let required_files = vec!["Nickel-pkg.ncl", "mod.ncl"];
        for required_file in required_files {
            if !package_dir.join(required_file).exists() {
                validation_errors.push(format!(
                    "Package {} missing required file: {}",
                    package_name, required_file
                ));
            }
        }

        // Check that Nickel-pkg.ncl has proper structure
        if let Ok(manifest_content) = std::fs::read_to_string(package_dir.join("Nickel-pkg.ncl")) {
            let required_fields = vec!["name", "version", "description", "authors"];
            for field in required_fields {
                if !manifest_content.contains(field) {
                    validation_errors.push(format!(
                        "Package {} manifest missing field: {}",
                        package_name, field
                    ));
                }
            }

            // Check that manifest mentions it was generated by Amalgam
            if !manifest_content.contains("Amalgam") && !manifest_content.contains("amalgam") {
                validation_errors.push(format!(
                    "Package {} manifest should mention it was generated by Amalgam",
                    package_name
                ));
            }
        }
    }

    if !validation_errors.is_empty() {
        return Err(format!(
            "Package manifest validation failed with {} errors:\n{}",
            validation_errors.len(),
            validation_errors.join("\n")
        ).into());
    }
    Ok(())
}
