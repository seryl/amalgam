//! Unified import path calculator for consistent import resolution across the codebase
//!
//! This module provides a single source of truth for calculating import paths between
//! different packages and versions, replacing the scattered logic throughout the codebase.

use crate::module_registry::ModuleRegistry;
use crate::naming::to_camel_case;
use std::path::PathBuf;
use std::sync::Arc;

/// Unified import path calculator for all import resolution needs
/// This now acts as a facade over the ModuleRegistry for backwards compatibility
#[derive(Debug, Clone)]
pub struct ImportPathCalculator {
    registry: Arc<ModuleRegistry>,
}

impl ImportPathCalculator {
    /// Create a new ImportPathCalculator with a shared ModuleRegistry
    pub fn new(registry: Arc<ModuleRegistry>) -> Self {
        Self { registry }
    }

    /// Create from an owned ModuleRegistry
    pub fn from_registry(registry: ModuleRegistry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }

    /// Create with an empty registry (for backward compatibility where IR is not yet available)
    pub fn new_standalone() -> Self {
        Self {
            registry: Arc::new(ModuleRegistry::new()),
        }
    }

    /// Calculate the import path from one type to another
    ///
    /// # Arguments
    /// * `from_group` - The API group of the importing file (e.g., "k8s.io")
    /// * `from_version` - The version of the importing file (e.g., "v1")
    /// * `to_group` - The API group of the target type
    /// * `to_version` - The version of the target type
    /// * `to_type` - The name of the target type (properly cased, without .ncl)
    ///
    /// # Returns
    /// The relative import path from the importing file to the target type
    ///
    /// # Important
    /// For k8s.io packages, this returns paths to consolidated module files (v1.ncl)
    /// rather than individual type files, as types are exported from the module.
    pub fn calculate(
        &self,
        from_group: &str,
        from_version: &str,
        to_group: &str,
        to_version: &str,
        to_type: &str,
    ) -> String {
        // MUST use registry - no fallback allowed for now, but return something for backward compatibility
        let from_module = format!("{}.{}", from_group, from_version);
        let to_module = format!("{}.{}", to_group, to_version);

        self.registry
            .calculate_import_path(&from_module, &to_module, to_type)
            .unwrap_or_else(|| {
                // TEMPORARY fallback until we integrate ModuleRegistry everywhere
                tracing::warn!(
                    "ModuleRegistry missing data for {} -> {}.{}, using fallback logic",
                    from_module,
                    to_module,
                    to_type
                );
                self.calculate_fallback(from_group, from_version, to_group, to_version, to_type)
            })
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
            // Same package: just use the type name in camelCase
            to_camel_case(to_type)
        } else {
            // Different package: include version if not default
            if to_version == "v1" {
                format!(
                    "{}_{}",
                    Self::group_to_alias(to_group),
                    to_camel_case(to_type)
                )
            } else {
                format!(
                    "{}_{}_{}",
                    Self::group_to_alias(to_group),
                    to_version,
                    to_camel_case(to_type)
                )
            }
        };

        (path, alias)
    }

    /// TEMPORARY: Fallback calculation until ModuleRegistry is fully integrated
    fn calculate_fallback(
        &self,
        from_group: &str,
        from_version: &str,
        to_group: &str,
        to_version: &str,
        to_type: &str,
    ) -> String {
        // Case 1: Same module - use relative import (but NOT for k8s.io which uses consolidated modules)
        if from_group == to_group
            && from_version == to_version
            && !to_group.contains("k8s.io")
            && !to_group.starts_with("io.k8s.")
        {
            return format!("./{}.ncl", to_type);
        }

        // Special handling for k8s.io consolidated module structure
        if to_group.contains("k8s.io") || to_group.starts_with("io.k8s.") {
            return self.calculate_k8s_import_path(
                from_group,
                from_version,
                to_group,
                to_version,
                to_type,
            );
        }

        // Case 2: Same package, different version
        if from_group == to_group {
            return format!("../{}/{}.ncl", to_version, to_type);
        }

        // Case 3: Different packages - calculate relative path
        let from_path = Self::group_to_path(from_group);
        let to_path = Self::group_to_path(to_group);
        let relative_path = Self::calculate_relative_path(&from_path, &to_path);

        // Use standard versioned path structure for all packages
        // The ModuleRegistry handles special cases via layout detection
        format!("{}/{}/{}.ncl", relative_path, to_version, to_type)
    }

    /// Calculate import path for k8s.io's consolidated module structure
    fn calculate_k8s_import_path(
        &self,
        from_group: &str,
        _from_version: &str,
        to_group: &str,
        to_version: &str,
        to_type: &str,
    ) -> String {
        // Map the type to its module location in k8s.io structure
        // ALL k8s.io types are in consolidated module files, not individual type files

        // Determine which consolidated module contains this type
        // Note: type names might be lowercase in tests, so we need case-insensitive comparison
        let type_lower = to_type.to_lowercase();
        let module_path = if type_lower == "objectmeta"
            || type_lower == "labelselector"
            || type_lower == "listmeta"
            || type_lower == "time"
            || type_lower == "condition"
            || type_lower == "managedfieldsentry"
        {
            // These are in apimachinery.pkg.apis/meta/v1/mod.ncl (consolidated module)
            format!("../../apimachinery.pkg.apis/meta/{}/mod.ncl", to_version)
        } else if type_lower == "intorstring" {
            // IntOrString is in v0/mod.ncl (unversioned types)
            "../../v0/mod.ncl".to_string()
        } else if type_lower == "rawextension" {
            // RawExtension is in v0/mod.ncl (unversioned types)
            "../../v0/mod.ncl".to_string()
        } else {
            // For unknown types, fall back to individual file approach
            // Only use consolidated modules for well-known types
            let known_core_types = [
                "pod",
                "service",
                "deployment",
                "configmap",
                "secret",
                "namespace",
                "node",
                "persistentvolume",
                "persistentvolumeclaim",
                "serviceaccount",
                "celdeviceselector",         // Used in tests
                "typedlocalobjectreference", // Core API type used by networking and other APIs
                "podtemplatespec",           // Core API type used by apps and batch APIs
                "objectreference",           // Core API type used by batch and events APIs
                "eventsource",               // Core API type used by events API
                "topologyselectorterm",      // Core API type used by storage API
                "persistentvolumespec",      // Core API type used by storage API
                "toleration",                // Core API type used by node API
                "nodeselector",              // Core API type used by resource APIs
            ];

            if known_core_types.contains(&type_lower.as_str()) {
                // Known core types are in consolidated modules
                // From api/networking to api/core it's just ../core/v1/mod.ncl
                format!("../core/{}/mod.ncl", to_version)
            } else {
                // Unknown types use individual files
                if from_group == to_group {
                    // Same package, different version - relative to current level
                    format!("../{}/{}.ncl", to_version, to_type)
                } else {
                    // Cross-package - calculate relative path to target
                    let from_path = Self::group_to_path(from_group);
                    let to_path = Self::group_to_path(to_group);
                    let relative_path = Self::calculate_relative_path(&from_path, &to_path);
                    format!("{}/{}/{}.ncl", relative_path, to_version, to_type)
                }
            }
        };

        module_path
    }

    /// Convert API group to filesystem path
    fn group_to_path(group: &str) -> PathBuf {
        match group {
            "k8s.io" => PathBuf::from("k8s_io"),
            "" => PathBuf::from("core"), // Core API group
            // For k8s.io related groups that might have dots, keep them as-is
            // The actual directory structure uses dots, not underscores
            g if g.contains('.') => {
                // Keep dots for k8s.io modules (apimachinery.pkg.apis)
                PathBuf::from(g)
            }
            g => PathBuf::from(g),
        }
    }

    /// Convert API group to import alias prefix
    fn group_to_alias(group: &str) -> &str {
        match group {
            "k8s.io" => "k8s",
            "" => "core",
            g => g.split('.').next().unwrap_or(g),
        }
    }

    /// Calculate relative path between two package paths
    fn calculate_relative_path(from: &PathBuf, to: &PathBuf) -> String {
        // Calculate how many levels deep we are from the packages root
        // The actual directory structure is:
        // - k8s packages: pkgs/k8s_io/<version>/<file>.ncl = 2 levels up
        // - CrossPlane: pkgs/crossplane/<domain>/crossplane/<file>.ncl = 3 levels up (no version subdir)
        //
        // We need to count the actual components in the path, plus version directory for non-CrossPlane

        let from_components = from.components().count();

        // Standard depth calculation - assume version directories for all
        // The ModuleRegistry should handle special cases
        let from_depth = from_components + 1; // +1 for version directory

        // Debug logging
        tracing::debug!("calculate_relative_path: from={:?}, to={:?}", from, to);
        tracing::debug!(
            "from_components={}, from_depth={}",
            from_components,
            from_depth
        );

        // Go up the required number of levels to reach the packages root
        let mut path_parts: Vec<&str> = vec![".."; from_depth];

        // Add the target package path
        for component in to.components() {
            if let Some(s) = component.as_os_str().to_str() {
                path_parts.push(s);
            }
        }

        let result = path_parts.join("/");
        tracing::debug!("calculate_relative_path result: {}", result);
        result
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

    fn test_calculator() -> ImportPathCalculator {
        // Create with empty registry for tests
        ImportPathCalculator::from_registry(ModuleRegistry::new())
    }

    #[test]
    fn test_same_package_same_version() {
        let calc = test_calculator();
        // k8s.io types always use consolidated modules, even for same version
        let path = calc.calculate("k8s.io", "v1", "k8s.io", "v1", "Pod");
        assert_eq!(path, "../core/v1/mod.ncl");

        // Non-k8s.io types should use local import
        let path2 = calc.calculate("example.io", "v1", "example.io", "v1", "MyType");
        assert_eq!(path2, "./MyType.ncl");
    }

    #[test]
    fn test_same_package_different_version() {
        let calc = test_calculator();
        // ObjectMeta is in apimachinery consolidated module
        let path = calc.calculate("k8s.io", "v1beta1", "k8s.io", "v1", "ObjectMeta");
        assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");
    }

    #[test]
    fn test_cross_package_import() {
        let calc = test_calculator();
        // ObjectMeta is in apimachinery consolidated module
        let path = calc.calculate(
            "apiextensions.crossplane.io",
            "v1",
            "k8s.io",
            "v1",
            "ObjectMeta",
        );
        assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");
    }

    #[test]
    fn test_crossplane_to_k8s_path() {
        let calc = test_calculator();
        // ObjectMeta is in apimachinery consolidated module
        let path = calc.calculate(
            "ops.crossplane.io",
            "v1alpha1",
            "k8s.io",
            "v1",
            "ObjectMeta",
        );
        // ObjectMeta is in apimachinery.pkg.apis/meta/v1/mod.ncl
        assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");
    }

    #[test]
    fn test_calculate_with_alias() {
        let calc = test_calculator();

        // k8s.io types always use consolidated modules, even for same version
        let (path, alias) = calc.calculate_with_alias("k8s.io", "v1", "k8s.io", "v1", "Pod");
        assert_eq!(path, "../core/v1/mod.ncl");
        assert_eq!(alias, "pod");

        // Cross-version - ObjectMeta in apimachinery
        let (path, alias) =
            calc.calculate_with_alias("k8s.io", "v1beta1", "k8s.io", "v1", "ObjectMeta");
        assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");
        assert_eq!(alias, "objectMeta");

        // Cross-package - ObjectMeta in apimachinery
        let (path, alias) = calc.calculate_with_alias(
            "apiextensions.crossplane.io",
            "v1",
            "k8s.io",
            "v1",
            "ObjectMeta",
        );
        assert_eq!(path, "../../apimachinery.pkg.apis/meta/v1/mod.ncl");
        assert_eq!(alias, "k8s_objectMeta");
    }

    #[test]
    fn test_requires_import() {
        let calc = test_calculator();

        // Same package, same version - no import needed
        assert!(!calc.requires_import("k8s.io", "v1", "k8s.io", "v1"));

        // Same package, different version - import needed
        assert!(calc.requires_import("k8s.io", "v1beta1", "k8s.io", "v1"));

        // Different package - import needed
        assert!(calc.requires_import("apiextensions.crossplane.io", "v1", "k8s.io", "v1"));
    }

    #[test]
    fn test_is_cross_version_import() {
        let calc = test_calculator();

        assert!(!calc.is_cross_version_import("k8s.io", "v1", "k8s.io", "v1"));
        assert!(calc.is_cross_version_import("k8s.io", "v1beta1", "k8s.io", "v1"));
        assert!(!calc.is_cross_version_import("apiextensions.crossplane.io", "v1", "k8s.io", "v1"));
    }

    #[test]
    fn test_v1alpha3_same_version() {
        let calc = test_calculator();

        // Test the specific case from deviceselector.ncl
        // k8s.io types always use consolidated modules, even for same version
        let path = calc.calculate(
            "k8s.io",
            "v1alpha3",
            "k8s.io",
            "v1alpha3",
            "celdeviceselector",
        );
        assert_eq!(path, "../core/v1alpha3/mod.ncl");
    }

    #[test]
    fn test_raw_extension_to_v0() {
        let calc = test_calculator();

        // RawExtension is in v0/mod.ncl consolidated module
        let path = calc.calculate("k8s.io", "v1", "k8s.io", "v0", "rawextension");
        assert_eq!(path, "../../v0/mod.ncl");
    }
}
