//! Package index management

use crate::package::Package;
#[allow(unused_imports)]
use crate::package::PackageMetadata;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Index entry for a package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub name: String,
    pub versions: Vec<VersionEntry>,
    pub latest: String,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Version entry in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub version: String,
    pub checksum: String,
    pub dependencies: Vec<DependencyEntry>,
    pub published_at: DateTime<Utc>,
    pub yanked: bool,
    pub path: String,
}

/// Dependency entry in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEntry {
    pub name: String,
    pub version_req: String,
    pub optional: bool,
    pub features: Vec<String>,
}

/// Package index for the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageIndex {
    pub packages: IndexMap<String, IndexEntry>,
    pub categories: HashMap<String, Vec<String>>,
    pub updated_at: DateTime<Utc>,
    pub version: String,
}

impl PackageIndex {
    /// Create a new empty index
    pub fn new() -> Self {
        Self {
            packages: IndexMap::new(),
            categories: HashMap::new(),
            updated_at: Utc::now(),
            version: "1.0.0".to_string(),
        }
    }

    /// Load index from a JSON file
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read index from {:?}", path))?;

        let index: Self =
            serde_json::from_str(&content).with_context(|| "Failed to parse index JSON")?;

        Ok(index)
    }

    /// Save index to a JSON file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content =
            serde_json::to_string_pretty(self).with_context(|| "Failed to serialize index")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {:?}", parent))?;
        }

        fs::write(path, content).with_context(|| format!("Failed to write index to {:?}", path))?;

        Ok(())
    }

    /// Add a package to the index
    pub fn add_package(&mut self, package: Package) -> Result<()> {
        let checksum = calculate_checksum(&package)?;

        let version_entry = VersionEntry {
            version: package.metadata.version.clone(),
            checksum,
            dependencies: package
                .metadata
                .dependencies
                .iter()
                .map(|dep| DependencyEntry {
                    name: dep.name.clone(),
                    version_req: dep.version_req.clone(),
                    optional: dep.optional,
                    features: dep.features.clone(),
                })
                .collect(),
            published_at: Utc::now(),
            yanked: false,
            path: format!("{}/{}", package.metadata.name, package.metadata.version),
        };

        if let Some(entry) = self.packages.get_mut(&package.metadata.name) {
            // Update existing package
            entry.versions.push(version_entry);
            entry.latest = package.metadata.version.clone();
            entry.updated_at = Utc::now();

            if let Some(desc) = &package.metadata.description {
                entry.description = Some(desc.clone());
            }
        } else {
            // Add new package
            let entry = IndexEntry {
                name: package.metadata.name.clone(),
                versions: vec![version_entry],
                latest: package.metadata.version.clone(),
                description: package.metadata.description.clone(),
                categories: package.metadata.categories.clone(),
                keywords: package.metadata.keywords.clone(),
                homepage: package.metadata.homepage.clone(),
                repository: package.metadata.repository.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            self.packages.insert(package.metadata.name.clone(), entry);

            // Update categories index
            for category in &package.metadata.categories {
                self.categories
                    .entry(category.clone())
                    .or_default()
                    .push(package.metadata.name.clone());
            }
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Find a package by name
    pub fn find_package(&self, name: &str) -> Option<&IndexEntry> {
        self.packages.get(name)
    }

    /// Find packages by category
    pub fn find_by_category(&self, category: &str) -> Vec<&IndexEntry> {
        self.categories
            .get(category)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| self.packages.get(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search packages by keyword
    pub fn search(&self, query: &str) -> Vec<&IndexEntry> {
        let query_lower = query.to_lowercase();

        self.packages
            .values()
            .filter(|entry| {
                entry.name.to_lowercase().contains(&query_lower)
                    || entry
                        .description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || entry
                        .keywords
                        .iter()
                        .any(|k| k.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get all package names
    pub fn package_names(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }

    /// Get statistics about the index
    pub fn stats(&self) -> IndexStats {
        let total_packages = self.packages.len();
        let total_versions = self.packages.values().map(|e| e.versions.len()).sum();

        let categories: HashSet<_> = self
            .packages
            .values()
            .flat_map(|e| e.categories.iter())
            .cloned()
            .collect();

        IndexStats {
            total_packages,
            total_versions,
            total_categories: categories.len(),
            updated_at: self.updated_at,
        }
    }
}

/// Index statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_packages: usize,
    pub total_versions: usize,
    pub total_categories: usize,
    pub updated_at: DateTime<Utc>,
}

/// Calculate checksum for a package
fn calculate_checksum(package: &Package) -> Result<String> {
    use sha2::{Digest, Sha256};

    let json = serde_json::to_string(package)
        .with_context(|| "Failed to serialize package for checksum")?;

    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    let result = hasher.finalize();

    Ok(hex::encode(result))
}

impl Default for PackageIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_operations() {
        let mut index = PackageIndex::new();

        let package = Package {
            metadata: PackageMetadata {
                name: "test-package".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test package".to_string()),
                categories: vec!["testing".to_string()],
                keywords: vec!["test".to_string()],
                homepage: None,
                repository: None,
                dependencies: vec![],
            },
            content: HashMap::new(),
        };

        index.add_package(package).unwrap();

        assert!(index.find_package("test-package").is_some());
        assert_eq!(index.find_by_category("testing").len(), 1);
        assert_eq!(index.search("test").len(), 1);
    }
}
