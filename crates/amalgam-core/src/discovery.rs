//! Filesystem discovery for auto-detecting module layouts
//!
//! This module provides intelligent detection of module organization patterns
//! by examining the actual filesystem structure, looking for version directories
//! and namespace partitioning.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Detected structure of a module package
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleStructure {
    /// Whether the package uses namespace directories (e.g., apiextensions.crossplane.io/)
    pub has_namespaces: bool,

    /// Whether the package uses version directories (e.g., v1/, v1beta1/)
    pub has_versions: bool,

    /// List of detected namespace directories
    pub namespaces: Vec<String>,

    /// List of detected version directories
    pub versions: Vec<String>,

    /// The depth at which types are found (for path calculation)
    pub type_depth: usize,
}

impl ModuleStructure {
    /// Detect the structure of a package by examining its filesystem
    pub fn detect(root: &Path) -> Self {
        let mut structure = Self {
            has_namespaces: false,
            has_versions: false,
            namespaces: Vec::new(),
            versions: Vec::new(),
            type_depth: 0,
        };

        // Look for version directories at the root level
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if Self::is_version_dir(name) {
                        structure.versions.push(name.to_string());
                        structure.has_versions = true;
                    } else if entry.path().is_dir() && name.contains('.') {
                        // Might be a namespace directory like apiextensions.crossplane.io
                        structure.namespaces.push(name.to_string());
                        structure.has_namespaces = true;

                        // Check if there are version directories inside the namespace
                        if let Ok(sub_entries) = std::fs::read_dir(entry.path()) {
                            for sub_entry in sub_entries.flatten() {
                                if let Some(sub_name) = sub_entry.file_name().to_str() {
                                    if Self::is_version_dir(sub_name) {
                                        if !structure.versions.contains(&sub_name.to_string()) {
                                            structure.versions.push(sub_name.to_string());
                                        }
                                        structure.has_versions = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Calculate type depth based on structure
        structure.type_depth = structure.calculate_type_depth();

        structure
    }

    /// Check if a directory name looks like a version
    fn is_version_dir(name: &str) -> bool {
        // Match patterns like v1, v1beta1, v1alpha2, v2, etc.
        if !name.starts_with('v') {
            return false;
        }

        let rest = &name[1..];

        // Check for pure version numbers (v1, v2, v10)
        if rest.chars().all(|c| c.is_ascii_digit()) && !rest.is_empty() {
            return true;
        }

        // Check for alpha/beta versions (v1alpha1, v1beta2)
        if let Some(num_end) = rest.find(|c: char| !c.is_ascii_digit()) {
            let num_part = &rest[..num_end];
            let suffix = &rest[num_end..];

            if !num_part.is_empty()
                && num_part.chars().all(|c| c.is_ascii_digit())
                && (suffix.starts_with("alpha") || suffix.starts_with("beta"))
            {
                let version_suffix = if let Some(stripped) = suffix.strip_prefix("alpha") {
                    stripped
                } else if let Some(stripped) = suffix.strip_prefix("beta") {
                    stripped
                } else {
                    suffix
                };

                // Should be followed by a number or nothing
                return version_suffix.is_empty()
                    || version_suffix.chars().all(|c| c.is_ascii_digit());
            }
        }

        false
    }

    /// Calculate how deep type files are in the structure
    fn calculate_type_depth(&self) -> usize {
        let mut depth = 0;

        if self.has_namespaces {
            depth += 1; // Namespace directory
            if self.namespaces.iter().any(|ns| ns.contains('.')) {
                depth += 1; // Additional nesting for dotted namespaces
            }
        }

        if self.has_versions {
            depth += 1; // Version directory
        }

        depth
    }

    /// Determine the appropriate ModuleLayout based on detected structure
    pub fn to_layout(&self) -> super::module_registry::ModuleLayout {
        use super::module_registry::ModuleLayout;

        // Check if we have a mix of version and non-version directories at root
        let has_mixed_root =
            self.has_versions && self.namespaces.iter().any(|ns| !Self::is_version_dir(ns));

        if has_mixed_root {
            // K8s pattern: both versions (v1, v2) and namespaces (resource) at root
            ModuleLayout::MixedRoot
        } else {
            match (self.has_namespaces, self.has_versions) {
                (true, true) => {
                    // Both namespaces and versions
                    // TODO: Detect if it's ApiGroupVersioned vs NamespacedVersioned
                    // by checking if versions are inside namespace dirs
                    ModuleLayout::NamespacedVersioned
                }
                (true, false) => ModuleLayout::NamespacedFlat,
                (false, true) => ModuleLayout::MixedRoot, // Just versions = MixedRoot
                (false, false) => ModuleLayout::Flat,
            }
        }
    }

    /// Get the latest stable version from detected versions
    pub fn get_latest_version(&self) -> Option<String> {
        if self.versions.is_empty() {
            return None;
        }

        // Sort versions by stability and recency
        let mut sorted_versions = self.versions.clone();
        sorted_versions.sort_by(|a, b| Self::compare_versions(a, b));

        sorted_versions.last().cloned()
    }

    /// Compare two version strings for precedence
    fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        // Extract version parts
        let parse_version = |v: &str| -> (u32, &str, u32) {
            if !v.starts_with('v') {
                return (0, "", 0);
            }

            let v = &v[1..];
            let num_end = v.find(|c: char| !c.is_ascii_digit()).unwrap_or(v.len());
            let main_version: u32 = v[..num_end].parse().unwrap_or(0);

            if num_end < v.len() {
                let suffix = &v[num_end..];
                if let Some(stripped) = suffix.strip_prefix("alpha") {
                    let sub_version: u32 = stripped.parse().unwrap_or(0);
                    return (main_version, "alpha", sub_version);
                } else if let Some(stripped) = suffix.strip_prefix("beta") {
                    let sub_version: u32 = stripped.parse().unwrap_or(0);
                    return (main_version, "beta", sub_version);
                }
            }

            (main_version, "stable", 0)
        };

        let (a_main, a_suffix, a_sub) = parse_version(a);
        let (b_main, b_suffix, b_sub) = parse_version(b);

        // Compare main version first
        match a_main.cmp(&b_main) {
            Ordering::Equal => {
                // Same main version, compare stability
                match (a_suffix, b_suffix) {
                    ("alpha", "alpha") => a_sub.cmp(&b_sub),
                    ("beta", "beta") => a_sub.cmp(&b_sub),
                    ("stable", "stable") => Ordering::Equal,
                    ("alpha", _) => Ordering::Less,
                    (_, "alpha") => Ordering::Greater,
                    ("beta", "stable") => Ordering::Less,
                    ("stable", "beta") => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_version_dir() {
        assert!(ModuleStructure::is_version_dir("v1"));
        assert!(ModuleStructure::is_version_dir("v2"));
        assert!(ModuleStructure::is_version_dir("v10"));
        assert!(ModuleStructure::is_version_dir("v1alpha1"));
        assert!(ModuleStructure::is_version_dir("v1alpha"));
        assert!(ModuleStructure::is_version_dir("v1beta1"));
        assert!(ModuleStructure::is_version_dir("v1beta2"));
        assert!(ModuleStructure::is_version_dir("v2alpha1"));

        assert!(!ModuleStructure::is_version_dir("v"));
        assert!(!ModuleStructure::is_version_dir("version1"));
        assert!(!ModuleStructure::is_version_dir("1"));
        assert!(!ModuleStructure::is_version_dir("v1gamma"));
        assert!(!ModuleStructure::is_version_dir("v1alphabeta"));
        assert!(!ModuleStructure::is_version_dir("resource"));
        assert!(!ModuleStructure::is_version_dir("core"));
    }

    #[test]
    fn test_version_comparison() {
        use std::cmp::Ordering;
        use ModuleStructure as MS;

        assert_eq!(MS::compare_versions("v1alpha1", "v1alpha2"), Ordering::Less);
        assert_eq!(MS::compare_versions("v1alpha1", "v1beta1"), Ordering::Less);
        assert_eq!(MS::compare_versions("v1beta1", "v1"), Ordering::Less);
        assert_eq!(MS::compare_versions("v1", "v2"), Ordering::Less);
        assert_eq!(MS::compare_versions("v2alpha1", "v1"), Ordering::Greater);
        assert_eq!(MS::compare_versions("v1", "v1"), Ordering::Equal);
    }

    #[test]
    fn test_get_latest_version() {
        let mut structure = ModuleStructure {
            has_namespaces: false,
            has_versions: true,
            namespaces: vec![],
            versions: vec![
                "v1alpha1".to_string(),
                "v1beta1".to_string(),
                "v1".to_string(),
                "v2alpha1".to_string(),
            ],
            type_depth: 1,
        };

        assert_eq!(structure.get_latest_version(), Some("v2alpha1".to_string()));

        structure.versions.push("v2".to_string());
        assert_eq!(structure.get_latest_version(), Some("v2".to_string()));
    }
}
