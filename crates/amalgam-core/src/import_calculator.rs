//! Unified import path calculator for consistent import resolution across the codebase
//!
//! This module provides a single source of truth for calculating import paths between
//! different packages and versions, replacing the scattered logic throughout the codebase.

use std::path::PathBuf;

/// Unified import path calculator for all import resolution needs
#[derive(Debug, Clone, Default)]
pub struct ImportPathCalculator;

impl ImportPathCalculator {
    /// Create a new ImportPathCalculator instance
    pub fn new() -> Self {
        Self
    }

    /// Calculate the import path from one type to another
    ///
    /// # Arguments
    /// * `from_group` - The API group of the importing file (e.g., "k8s.io")
    /// * `from_version` - The version of the importing file (e.g., "v1")
    /// * `to_group` - The API group of the target type
    /// * `to_version` - The version of the target type
    /// * `to_type` - The name of the target type (lowercase, without .ncl)
    ///
    /// # Returns
    /// The relative import path from the importing file to the target type
    pub fn calculate(
        &self,
        from_group: &str,
        from_version: &str,
        to_group: &str,
        to_version: &str,
        to_type: &str,
    ) -> String {
        // Normalize type name to lowercase
        let type_name = to_type.to_lowercase();

        // Case 1: Same package, same version - use relative import
        if from_group == to_group && from_version == to_version {
            return format!("./{}.ncl", type_name);
        }

        // Case 2: Same package, different version - go up one level
        if from_group == to_group {
            return format!("../{}/{}.ncl", to_version, type_name);
        }

        // Case 3: Different packages - calculate relative path
        let from_path = Self::group_to_path(from_group);
        let to_path = Self::group_to_path(to_group);

        // Calculate relative path between packages
        let relative = Self::calculate_relative_path(&from_path, &to_path);

        // Append version and type
        format!("{}/{}/{}.ncl", relative, to_version, type_name)
    }

    /// Calculate import path with optional alias
    ///
    /// Returns a tuple of (import_path, suggested_alias)
    pub fn calculate_with_alias(
        &self,
        from_group: &str,
        from_version: &str,
        to_group: &str,
        to_version: &str,
        to_type: &str,
    ) -> (String, String) {
        let path = self.calculate(from_group, from_version, to_group, to_version, to_type);

        // Generate alias based on the context
        let alias = if from_group == to_group {
            // Same package: just use the type name
            to_type.to_lowercase()
        } else {
            // Different package: include version if not default
            if to_version == "v1" {
                format!(
                    "{}_{}",
                    Self::group_to_alias(to_group),
                    to_type.to_lowercase()
                )
            } else {
                format!(
                    "{}_{}_{}",
                    Self::group_to_alias(to_group),
                    to_version,
                    to_type.to_lowercase()
                )
            }
        };

        (path, alias)
    }

    /// Convert API group to filesystem path
    fn group_to_path(group: &str) -> PathBuf {
        match group {
            "k8s.io" => PathBuf::from("k8s_io"),
            "" => PathBuf::from("core"), // Core API group
            // CrossPlane groups have nested directory structures
            g if g.contains("crossplane.io") => {
                let mut path = PathBuf::from("crossplane");
                path.push(g);
                path.push("crossplane"); // The final directory is always "crossplane"
                path
            }
            g if g.contains('.') => {
                // Convert dots to underscores for filesystem compatibility
                PathBuf::from(g.replace('.', "_"))
            }
            g => PathBuf::from(g),
        }
    }

    /// Convert API group to import alias prefix
    fn group_to_alias(group: &str) -> &str {
        match group {
            "k8s.io" => "k8s",
            "apiextensions.crossplane.io" => "crossplane",
            "" => "core",
            g => g.split('.').next().unwrap_or(g),
        }
    }

    /// Calculate relative path between two package paths
    fn calculate_relative_path(from: &PathBuf, to: &PathBuf) -> String {
        // Calculate how many levels deep we are from the packages root
        // Examples:
        // - k8s.io: k8s_io/<version>/<file> = 2 levels up to reach pkgs/
        // - crossplane: crossplane/protection.crossplane.io/crossplane/<version>/<file> = 4 levels up to reach pkgs/
        
        // The depth is the number of components in the from path + 1 for the version directory
        // But we need to go up to reach the packages root, so we use the full path depth
        let from_depth = from.components().count() + 1; // +1 for version directory
        
        let mut path_parts = vec![];

        // Go up the required number of levels to reach the packages root
        // But we actually need one less level since we're calculating relative to the file location
        for _ in 0..(from_depth - 1) {
            path_parts.push("..");
        }

        // Add the target package path
        for component in to.components() {
            if let Some(s) = component.as_os_str().to_str() {
                path_parts.push(s);
            }
        }

        path_parts.join("/")
    }

    /// Check if a type reference requires an import
    pub fn requires_import(
        &self,
        from_group: &str,
        from_version: &str,
        to_group: &str,
        to_version: &str,
    ) -> bool {
        // Import is required if either group or version differs
        from_group != to_group || from_version != to_version
    }

    /// Determine if this is a cross-version import within the same package
    pub fn is_cross_version_import(
        &self,
        from_group: &str,
        from_version: &str,
        to_group: &str,
        to_version: &str,
    ) -> bool {
        from_group == to_group && from_version != to_version
    }

    /// Determine if this is a cross-package import
    pub fn is_cross_package_import(
        &self,
        from_group: &str,
        _from_version: &str,
        to_group: &str,
        _to_version: &str,
    ) -> bool {
        from_group != to_group
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_package_same_version() {
        let calc = ImportPathCalculator::new();
        let path = calc.calculate("k8s.io", "v1", "k8s.io", "v1", "pod");
        assert_eq!(path, "./pod.ncl");
    }

    #[test]
    fn test_same_package_different_version() {
        let calc = ImportPathCalculator::new();
        let path = calc.calculate("k8s.io", "v1beta1", "k8s.io", "v1", "objectmeta");
        assert_eq!(path, "../v1/objectmeta.ncl");
    }

    #[test]
    fn test_cross_package_import() {
        let calc = ImportPathCalculator::new();
        let path = calc.calculate(
            "apiextensions.crossplane.io",
            "v1",
            "k8s.io",
            "v1",
            "objectmeta",
        );
        assert!(path.contains("k8s_io"));
        assert!(path.contains("v1"));
        assert!(path.contains("objectmeta.ncl"));
    }

    #[test]
    fn test_crossplane_to_k8s_path() {
        let calc = ImportPathCalculator::new();
        // From a CrossPlane ops.crossplane.io package to k8s.io
        let path = calc.calculate(
            "ops.crossplane.io", 
            "crossplane", 
            "k8s.io", 
            "v1", 
            "objectmeta"
        );
        // Should be ../../../k8s_io/v1/objectmeta.ncl
        // Going up from: crossplane/ops.crossplane.io/crossplane/<version>/file.ncl
        // That's 3 levels up to reach pkgs/, then down to k8s_io/v1/
        assert_eq!(path, "../../../k8s_io/v1/objectmeta.ncl");
    }

    #[test]
    fn test_calculate_with_alias() {
        let calc = ImportPathCalculator::new();

        // Same package
        let (path, alias) = calc.calculate_with_alias("k8s.io", "v1", "k8s.io", "v1", "Pod");
        assert_eq!(path, "./pod.ncl");
        assert_eq!(alias, "pod");

        // Cross-version
        let (path, alias) =
            calc.calculate_with_alias("k8s.io", "v1beta1", "k8s.io", "v1", "ObjectMeta");
        assert_eq!(path, "../v1/objectmeta.ncl");
        assert_eq!(alias, "objectmeta");

        // Cross-package
        let (path, alias) = calc.calculate_with_alias(
            "apiextensions.crossplane.io",
            "v1",
            "k8s.io",
            "v1",
            "ObjectMeta",
        );
        assert!(path.contains("objectmeta.ncl"));
        assert_eq!(alias, "k8s_objectmeta");
    }

    #[test]
    fn test_requires_import() {
        let calc = ImportPathCalculator::new();

        // Same package, same version - no import needed
        assert!(!calc.requires_import("k8s.io", "v1", "k8s.io", "v1"));

        // Same package, different version - import needed
        assert!(calc.requires_import("k8s.io", "v1beta1", "k8s.io", "v1"));

        // Different package - import needed
        assert!(calc.requires_import("apiextensions.crossplane.io", "v1", "k8s.io", "v1"));
    }

    #[test]
    fn test_is_cross_version_import() {
        let calc = ImportPathCalculator::new();

        assert!(!calc.is_cross_version_import("k8s.io", "v1", "k8s.io", "v1"));
        assert!(calc.is_cross_version_import("k8s.io", "v1beta1", "k8s.io", "v1"));
        assert!(!calc.is_cross_version_import("apiextensions.crossplane.io", "v1", "k8s.io", "v1"));
    }

    #[test]
    fn test_v1alpha3_same_version() {
        let calc = ImportPathCalculator::new();

        // Test the specific case from deviceselector.ncl
        let path = calc.calculate(
            "k8s.io",
            "v1alpha3",
            "k8s.io",
            "v1alpha3",
            "celdeviceselector",
        );
        assert_eq!(path, "./celdeviceselector.ncl");
    }

    #[test]
    fn test_raw_extension_to_v0() {
        let calc = ImportPathCalculator::new();

        // RawExtension should import from v0
        let path = calc.calculate("k8s.io", "v1", "k8s.io", "v0", "rawextension");
        assert_eq!(path, "../v0/rawextension.ncl");
    }
}
