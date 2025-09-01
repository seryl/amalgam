//! Package mode handling for code generation
//!
//! This module provides generic package handling without special casing
//! for specific packages. All package detection is based on actual usage.

use amalgam_core::dependency_analyzer::DependencyAnalyzer;
use amalgam_core::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents how a package dependency should be resolved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    /// The package identifier (e.g., "github:seryl/nickel-pkgs/k8s-io")
    pub package_id: String,
    /// Version constraint (e.g., ">=1.31.0")
    pub version: String,
}

/// Determines how imports are generated in the output
#[derive(Debug, Clone)]
pub enum PackageMode {
    /// Generate relative file imports (default for local development)
    Relative,

    /// Generate package imports for nickel-mine
    Package {
        /// Map of external package dependencies discovered through analysis
        dependencies: HashMap<String, PackageDependency>,
        /// Dependency analyzer for automatic detection
        analyzer: DependencyAnalyzer,
    },

    /// Local development mode with local package paths
    LocalDevelopment {
        /// Map of package names to local paths
        local_paths: HashMap<String, PathBuf>,
    },
}

impl Default for PackageMode {
    fn default() -> Self {
        PackageMode::Relative
    }
}

impl PackageMode {
    /// Create a new package mode with automatic dependency detection
    pub fn new_with_analyzer(manifest_path: Option<&PathBuf>) -> Self {
        let mut analyzer = DependencyAnalyzer::new();

        // If we have a manifest, register known types from it
        if let Some(path) = manifest_path {
            let _ = analyzer.register_from_manifest(path);
        }

        PackageMode::Package {
            dependencies: HashMap::new(),
            analyzer,
        }
    }

    /// Analyze types to detect dependencies automatically
    pub fn analyze_and_update_dependencies(&mut self, types: &[Type], current_package: &str) {
        if let PackageMode::Package {
            analyzer,
            dependencies,
        } = self
        {
            analyzer.set_current_package(current_package);

            // Analyze all types to find external references
            let mut all_refs = std::collections::HashSet::new();
            for ty in types {
                let refs = analyzer.analyze_type(ty, current_package);
                all_refs.extend(refs);
            }

            // Determine required dependencies
            let detected_deps = analyzer.determine_dependencies(&all_refs);

            // Update our dependency map
            for dep in detected_deps {
                if !dependencies.contains_key(&dep.package_name) {
                    // Auto-generate package ID based on detected package
                    let base = std::env::var("NICKEL_PACKAGE_BASE")
                        .unwrap_or_else(|_| "github:seryl/nickel-pkgs".to_string());
                    let package_id = format!("{}/{}", base, &dep.package_name);

                    let version = if dep.is_core_type {
                        ">=1.31.0".to_string()
                    } else {
                        ">=0.1.0".to_string()
                    };

                    dependencies.insert(
                        dep.package_name.clone(),
                        PackageDependency {
                            package_id,
                            version,
                        },
                    );
                }
            }
        }
    }

    /// Convert an import path based on the package mode
    pub fn convert_import(&self, import_path: &str) -> String {
        match self {
            PackageMode::Relative => {
                // Keep as relative import
                import_path.to_string()
            }
            PackageMode::Package { .. } => {
                // Check if this import references an external package
                if let Some(package_name) = self.detect_package_from_path(import_path) {
                    // Convert to package import
                    format!("\"{}\"", package_name)
                } else {
                    // Keep as relative import within same package
                    import_path.to_string()
                }
            }
            PackageMode::LocalDevelopment { local_paths } => {
                // Check if this import references a local package
                for (package_name, local_path) in local_paths {
                    if import_path.contains(package_name) {
                        return local_path.to_string_lossy().to_string();
                    }
                }
                import_path.to_string()
            }
        }
    }

    /// Detect package name from an import path
    fn detect_package_from_path(&self, import_path: &str) -> Option<String> {
        // Look for package patterns in the path
        // This is based on path structure, not hardcoded names

        // Pattern: ../../../package_name/...
        if import_path.starts_with("../") {
            let parts: Vec<&str> = import_path.split('/').collect();
            // Find the first non-".." component
            for part in parts {
                if part != ".." && part != "." && !part.ends_with(".ncl") {
                    // This might be a package name
                    // Check if we know about this package
                    if let PackageMode::Package { dependencies, .. } = self {
                        if dependencies.contains_key(part) {
                            return Some(part.to_string());
                        }
                        // Also check common transformations
                        let normalized = part.replace('_', "-");
                        if dependencies.contains_key(&normalized) {
                            return Some(normalized);
                        }
                    }
                    break;
                }
            }
        }

        None
    }

    /// Generate import statements for detected dependencies
    pub fn generate_imports(&self, types: &[Type], current_package: &str) -> Vec<String> {
        match self {
            PackageMode::Package { analyzer, .. } => {
                // Use analyzer to detect and generate imports
                let mut analyzer = analyzer.clone();
                analyzer.set_current_package(current_package);

                let mut all_refs = std::collections::HashSet::new();
                for ty in types {
                    let refs = analyzer.analyze_type(ty, current_package);
                    all_refs.extend(refs);
                }

                let deps = analyzer.determine_dependencies(&all_refs);
                analyzer.generate_imports(&deps, true)
            }
            _ => Vec::new(),
        }
    }

    /// Add Nickel package manifest fields based on detected dependencies
    pub fn add_to_manifest(&self, content: &str, _package_name: &str) -> String {
        if let PackageMode::Package { dependencies, .. } = self {
            if !dependencies.is_empty() {
                let mut deps_str = String::from("  dependencies = {\n");
                for (dep_name, dep_info) in dependencies {
                    deps_str.push_str(&format!(
                        "    \"{}\" = \"{}\",\n",
                        dep_name, dep_info.version
                    ));
                }
                deps_str.push_str("  },\n");

                // Insert dependencies into manifest
                if content.contains("dependencies = {}") {
                    content.replace("dependencies = {}", &deps_str.trim_end())
                } else {
                    content.to_string()
                }
            } else {
                content.to_string()
            }
        } else {
            content.to_string()
        }
    }

    /// Get the dependency map (for testing and debugging)
    pub fn get_dependencies(&self) -> Option<&HashMap<String, PackageDependency>> {
        match self {
            PackageMode::Package { dependencies, .. } => Some(dependencies),
            _ => None,
        }
    }
}

/// Helper to create a basic Nickel package manifest
pub fn create_package_manifest(
    name: &str,
    version: &str,
    description: &str,
    keywords: Vec<String>,
    dependencies: HashMap<String, String>,
) -> String {
    let deps = if dependencies.is_empty() {
        "{}".to_string()
    } else {
        let entries: Vec<String> = dependencies
            .iter()
            .map(|(k, v)| format!("    \"{}\" = \"{}\"", k, v))
            .collect();
        format!("{{\n{}\n  }}", entries.join(",\n"))
    };

    format!(
        r#"{{
  name = "{}",
  version = "{}",
  description = "{}",
  
  keywords = [{}],
  
  dependencies = {},
  
  # Auto-generated by amalgam
  minimal_nickel_version = "1.9.0",
}} | std.package.Manifest
"#,
        name,
        version,
        description,
        keywords
            .iter()
            .map(|k| format!("\"{}\"", k))
            .collect::<Vec<_>>()
            .join(", "),
        deps
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_conversion_with_analyzer() {
        let mode = PackageMode::new_with_analyzer(None);

        // Test that imports are converted based on detected packages
        let import = "../../../k8s_io/v1/objectmeta.ncl";
        let converted = mode.convert_import(import);

        // Without registered dependencies, should stay as-is
        assert_eq!(converted, import);
    }

    #[test]
    fn test_package_manifest_generation() {
        let manifest = create_package_manifest(
            "test-package",
            "1.0.0",
            "Test package",
            vec!["test".to_string()],
            HashMap::new(),
        );

        assert!(manifest.contains("name = \"test-package\""));
        assert!(manifest.contains("version = \"1.0.0\""));
        assert!(manifest.contains("dependencies = {}"));
    }
}
