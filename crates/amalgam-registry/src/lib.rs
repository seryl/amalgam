//! Package registry and dependency management for Amalgam

pub mod index;
pub mod package;
pub mod resolver;
pub mod version;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub use index::{IndexEntry, PackageIndex};
pub use package::{Package, PackageBuilder, PackageDependency, PackageMetadata};
pub use resolver::{DependencyResolver, Resolution};
pub use version::{VersionConstraint, VersionRange};

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub name: String,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub cache_dir: String,
}

/// Main registry interface
pub struct Registry {
    _config: RegistryConfig,
    index: PackageIndex,
}

impl Registry {
    /// Create a new registry
    pub fn new(config: RegistryConfig) -> Result<Self> {
        let index = PackageIndex::new();
        Ok(Self {
            _config: config,
            index,
        })
    }

    /// Load registry from a directory
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let config = RegistryConfig {
            name: "local".to_string(),
            url: None,
            local_path: Some(path.to_string_lossy().to_string()),
            cache_dir: path.join(".cache").to_string_lossy().to_string(),
        };

        let index = PackageIndex::load_from_path(&path.join("index.json"))?;

        Ok(Self {
            _config: config,
            index,
        })
    }

    /// Add a package to the registry
    pub fn add_package(&mut self, package: Package) -> Result<()> {
        self.index.add_package(package)
    }

    /// Find a package by name
    pub fn find_package(&self, name: &str) -> Option<&IndexEntry> {
        self.index.find_package(name)
    }

    /// Resolve dependencies for a package
    pub fn resolve_dependencies(&self, package_name: &str, version: &str) -> Result<Resolution> {
        let mut resolver = DependencyResolver::new(&self.index);
        resolver.resolve(package_name, version)
    }

    /// Save the registry index
    pub fn save(&self, path: &Path) -> Result<()> {
        self.index.save(path)
    }

    /// Get package names
    pub fn package_names(&self) -> Vec<String> {
        self.index.package_names()
    }

    /// Search packages
    pub fn search(&self, query: &str) -> Vec<&IndexEntry> {
        self.index.search(query)
    }
}
