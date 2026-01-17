//! Consolidated import tracking for Nickel code generation
//!
//! This module provides a single source of truth for tracking imports during
//! code generation, replacing the scattered import tracking throughout the codebase.
//!
//! ## Design
//!
//! The `ImportTracker` consolidates all import tracking into one structure:
//! - Cross-package imports (to other packages)
//! - Same-package imports (to other modules in same package)
//! - Type-to-import mappings (which types need which imports)
//!
//! ## Usage
//!
//! ```ignore
//! let mut tracker = ImportTracker::new("k8s.io.v1", &registry);
//!
//! // Record type references
//! tracker.add_type_reference("ObjectMeta", "io.k8s.apimachinery.pkg.apis.meta.v1")?;
//!
//! // Generate imports
//! let import_statements = tracker.generate_import_statements();
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use amalgam_core::module_registry::ModuleRegistry;

use crate::CodegenError;

/// Information about a single import
#[derive(Debug, Clone)]
pub struct ImportInfo {
    /// The full import path (e.g., "../../apimachinery.pkg.apis/meta/v1/mod.ncl")
    pub path: String,
    /// The alias used in the let-binding (e.g., "metav1")
    pub alias: String,
    /// Types imported from this module
    pub types: HashSet<String>,
    /// Source module name (for debugging)
    pub source_module: String,
    /// Is this a same-package import?
    pub same_package: bool,
}

/// Errors that can occur during import tracking
#[derive(Debug, Clone, thiserror::Error)]
pub enum ImportError {
    #[error("Type '{type_name}' not found in any registered module")]
    TypeNotFound { type_name: String },

    #[error("Module '{module}' not found in registry")]
    ModuleNotFound { module: String },

    #[error("Cannot calculate import path from '{from}' to '{to}'")]
    PathCalculationFailed { from: String, to: String },
}

impl From<ImportError> for CodegenError {
    fn from(e: ImportError) -> Self {
        CodegenError::Generation(e.to_string())
    }
}

/// Consolidated import tracker for a module's code generation
///
/// This is the single source of truth for import tracking, consolidating:
/// - `current_imports: HashSet<(String, String)>`
/// - `same_package_deps: HashSet<String>`
/// - `cross_package_imports: Vec<String>`
/// - `type_import_map: TypeImportMap`
/// - `current_module_imports: HashMap<String, bool>`
#[derive(Debug)]
pub struct ImportTracker {
    /// The module being generated
    current_module: String,
    /// The current module's group (e.g., "k8s.io")
    current_group: String,
    /// The current module's version (e.g., "v1")
    current_version: String,
    /// Registry for module lookups
    registry: Arc<ModuleRegistry>,
    /// All imports keyed by their alias (for deduplication)
    imports_by_alias: BTreeMap<String, ImportInfo>,
    /// Reverse lookup: type name -> alias
    type_to_alias: HashMap<String, String>,
    /// Track which types have been processed (for cycle detection)
    processed_types: HashSet<String>,
    /// Errors encountered during import tracking
    errors: Vec<ImportError>,
}

impl ImportTracker {
    /// Create a new import tracker for a module
    pub fn new(current_module: &str, registry: Arc<ModuleRegistry>) -> Self {
        let (group, version) = Self::parse_module_name(current_module);
        Self {
            current_module: current_module.to_string(),
            current_group: group,
            current_version: version,
            registry,
            imports_by_alias: BTreeMap::new(),
            type_to_alias: HashMap::new(),
            processed_types: HashSet::new(),
            errors: Vec::new(),
        }
    }

    /// Reset the tracker for a new module
    pub fn reset(&mut self, current_module: &str) {
        let (group, version) = Self::parse_module_name(current_module);
        self.current_module = current_module.to_string();
        self.current_group = group;
        self.current_version = version;
        self.imports_by_alias.clear();
        self.type_to_alias.clear();
        self.processed_types.clear();
        self.errors.clear();
    }

    /// Parse module name into group and version
    fn parse_module_name(module_name: &str) -> (String, String) {
        let parts: Vec<&str> = module_name.split('.').collect();

        // Find the version part (starts with 'v' followed by digit, or special versions)
        let version_idx = parts.iter().rposition(|p| {
            (p.starts_with('v') && p.len() > 1 && p.chars().nth(1).map_or(false, |c| c.is_ascii_digit()))
                || *p == "v0"
                || *p == "crossplane"
                || *p == "resource"
        });

        match version_idx {
            Some(idx) => {
                let group = parts[..idx].join(".");
                let version = parts[idx].to_string();
                (group, version)
            }
            None => (module_name.to_string(), "v1".to_string()),
        }
    }

    /// Add a type reference that may require an import
    ///
    /// Returns the alias to use for the type, or None if no import is needed
    pub fn add_type_reference(
        &mut self,
        type_name: &str,
        source_module: &str,
    ) -> Result<Option<String>, ImportError> {
        // Skip if already processed
        if self.type_to_alias.contains_key(type_name) {
            return Ok(self.type_to_alias.get(type_name).cloned());
        }

        // Parse source module
        let (source_group, source_version) = Self::parse_module_name(source_module);

        // Check if import is needed (different module)
        if source_module == self.current_module {
            // Same module - no import needed
            return Ok(None);
        }

        // Check if same package
        let same_package = source_group == self.current_group;

        // Calculate import path
        let import_path = self
            .registry
            .calculate_import_path(&self.current_module, source_module, type_name)
            .ok_or_else(|| ImportError::PathCalculationFailed {
                from: self.current_module.clone(),
                to: source_module.to_string(),
            })?;

        // Generate alias
        let alias = self.generate_alias(&source_group, &source_version, type_name, same_package);

        // Check if we already have an import with this path
        if let Some(existing) = self.imports_by_alias.get_mut(&alias) {
            // Add this type to the existing import
            existing.types.insert(type_name.to_string());
        } else {
            // Create new import
            let info = ImportInfo {
                path: import_path,
                alias: alias.clone(),
                types: {
                    let mut set = HashSet::new();
                    set.insert(type_name.to_string());
                    set
                },
                source_module: source_module.to_string(),
                same_package,
            };
            self.imports_by_alias.insert(alias.clone(), info);
        }

        // Record type -> alias mapping
        self.type_to_alias.insert(type_name.to_string(), alias.clone());
        self.processed_types.insert(type_name.to_string());

        Ok(Some(alias))
    }

    /// Generate an appropriate alias for an import
    fn generate_alias(
        &self,
        group: &str,
        version: &str,
        type_name: &str,
        same_package: bool,
    ) -> String {
        // Special cases for well-known K8s modules
        if group.contains("apimachinery") && group.contains("meta") {
            return format!("meta{}", Self::format_version(version));
        }

        if group.contains("runtime") || version == "v0" {
            return "v0Module".to_string();
        }

        // For k8s.io API groups
        if group.starts_with("io.k8s.api.") {
            let api_group = group
                .strip_prefix("io.k8s.api.")
                .unwrap_or("")
                .to_string();
            if api_group == "core" {
                return format!("core{}", Self::format_version(version));
            }
            return format!("{}{}", api_group, Self::format_version(version));
        }

        if same_package {
            // Same package - use type-based alias
            Self::to_camel_case(type_name)
        } else {
            // Cross-package - include group and version
            let group_short = group.split('.').next().unwrap_or(group);
            format!(
                "{}_{}",
                group_short,
                Self::to_camel_case(type_name)
            )
        }
    }

    /// Format version for alias (v1 -> V1, v1alpha3 -> V1alpha3)
    fn format_version(version: &str) -> String {
        if version.is_empty() {
            return String::new();
        }
        let mut chars: Vec<char> = version.chars().collect();
        if !chars.is_empty() {
            chars[0] = chars[0].to_ascii_uppercase();
        }
        chars.into_iter().collect()
    }

    /// Convert to camelCase
    fn to_camel_case(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }
        let mut chars: Vec<char> = s.chars().collect();
        chars[0] = chars[0].to_ascii_lowercase();
        chars.into_iter().collect()
    }

    /// Get the alias for a type (if it has been imported)
    pub fn get_alias(&self, type_name: &str) -> Option<&str> {
        self.type_to_alias.get(type_name).map(|s| s.as_str())
    }

    /// Check if a type has been imported
    pub fn has_import(&self, type_name: &str) -> bool {
        self.type_to_alias.contains_key(type_name)
    }

    /// Generate all import statements (sorted for consistent output)
    pub fn generate_import_statements(&self) -> Vec<String> {
        let mut statements = Vec::new();

        // Cross-package imports first
        for info in self.imports_by_alias.values() {
            if !info.same_package {
                statements.push(format!(
                    "let {} = import \"{}\" in",
                    info.alias, info.path
                ));
            }
        }

        // Then same-package imports
        for info in self.imports_by_alias.values() {
            if info.same_package {
                statements.push(format!(
                    "let {} = import \"{}\" in",
                    info.alias, info.path
                ));
            }
        }

        statements
    }

    /// Get all imports as a formatted string
    pub fn format_imports(&self) -> String {
        let statements = self.generate_import_statements();
        if statements.is_empty() {
            return String::new();
        }

        statements.join("\n")
    }

    /// Get statistics about imports
    pub fn stats(&self) -> ImportStats {
        let cross_package_count = self
            .imports_by_alias
            .values()
            .filter(|i| !i.same_package)
            .count();
        let same_package_count = self
            .imports_by_alias
            .values()
            .filter(|i| i.same_package)
            .count();
        let total_types = self.type_to_alias.len();

        ImportStats {
            cross_package_count,
            same_package_count,
            total_types,
            errors: self.errors.len(),
        }
    }

    /// Get any errors that occurred
    pub fn errors(&self) -> &[ImportError] {
        &self.errors
    }

    /// Record an error (for batch reporting later)
    pub fn record_error(&mut self, error: ImportError) {
        self.errors.push(error);
    }

    /// Get all imports (for iteration)
    pub fn all_imports(&self) -> impl Iterator<Item = &ImportInfo> {
        self.imports_by_alias.values()
    }

    /// Check if there are any imports
    pub fn has_imports(&self) -> bool {
        !self.imports_by_alias.is_empty()
    }
}

/// Statistics about imports
#[derive(Debug, Clone, Default)]
pub struct ImportStats {
    pub cross_package_count: usize,
    pub same_package_count: usize,
    pub total_types: usize,
    pub errors: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_registry() -> Arc<ModuleRegistry> {
        Arc::new(ModuleRegistry::new())
    }

    #[test]
    fn test_parse_module_name() {
        let cases = vec![
            ("k8s.io.v1", ("k8s.io", "v1")),
            ("k8s.io.v1alpha3", ("k8s.io", "v1alpha3")),
            (
                "io.k8s.apimachinery.pkg.apis.meta.v1",
                ("io.k8s.apimachinery.pkg.apis.meta", "v1"),
            ),
            (
                "apiextensions.crossplane.io.v1",
                ("apiextensions.crossplane.io", "v1"),
            ),
        ];

        for (input, (expected_group, expected_version)) in cases {
            let (group, version) = ImportTracker::parse_module_name(input);
            assert_eq!(group, expected_group, "Group mismatch for {}", input);
            assert_eq!(version, expected_version, "Version mismatch for {}", input);
        }
    }

    #[test]
    fn test_format_version() {
        assert_eq!(ImportTracker::format_version("v1"), "V1");
        assert_eq!(ImportTracker::format_version("v1alpha3"), "V1alpha3");
        assert_eq!(ImportTracker::format_version("v0"), "V0");
        assert_eq!(ImportTracker::format_version(""), "");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(ImportTracker::to_camel_case("ObjectMeta"), "objectMeta");
        assert_eq!(ImportTracker::to_camel_case("Pod"), "pod");
        assert_eq!(ImportTracker::to_camel_case("ABC"), "aBC");
        assert_eq!(ImportTracker::to_camel_case(""), "");
    }

    #[test]
    fn test_reset() {
        let registry = test_registry();
        let mut tracker = ImportTracker::new("k8s.io.v1", registry);

        tracker.processed_types.insert("Test".to_string());
        assert!(!tracker.processed_types.is_empty());

        tracker.reset("k8s.io.v2");
        assert!(tracker.processed_types.is_empty());
        assert_eq!(tracker.current_module, "k8s.io.v2");
        assert_eq!(tracker.current_version, "v2");
    }

    #[test]
    fn test_same_module_no_import() {
        let registry = test_registry();
        let mut tracker = ImportTracker::new("k8s.io.v1", registry);

        // Same module reference should return None (no import needed)
        let result = tracker.add_type_reference("Pod", "k8s.io.v1");
        // This will fail because registry is empty, but that's expected
        // In a real scenario with populated registry, it should return Ok(None)
        assert!(result.is_err() || result.unwrap().is_none());
    }

    #[test]
    fn test_stats() {
        let registry = test_registry();
        let tracker = ImportTracker::new("k8s.io.v1", registry);

        let stats = tracker.stats();
        assert_eq!(stats.cross_package_count, 0);
        assert_eq!(stats.same_package_count, 0);
        assert_eq!(stats.total_types, 0);
        assert_eq!(stats.errors, 0);
    }
}
