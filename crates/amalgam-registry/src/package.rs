//! Package structure and metadata

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Package metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Vec<PackageDependency>,
}

/// Package dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    pub name: String,
    pub version_req: String,
    pub optional: bool,
    pub features: Vec<String>,
}

/// Complete package structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub metadata: PackageMetadata,
    pub content: HashMap<String, String>, // file path -> content
}

impl Package {
    /// Create a new package
    pub fn new(metadata: PackageMetadata) -> Self {
        Self {
            metadata,
            content: HashMap::new(),
        }
    }

    /// Load package from a directory
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let metadata_path = path.join("package.toml");
        let metadata_content = fs::read_to_string(&metadata_path)?;
        let metadata: PackageMetadata = toml::from_str(&metadata_content)?;

        let mut content = HashMap::new();

        // Load all .ncl files
        load_nickel_files(path, path, &mut content)?;

        Ok(Self { metadata, content })
    }

    /// Save package to a directory
    pub fn save(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)?;

        // Save metadata
        let metadata_path = path.join("package.toml");
        let metadata_content = toml::to_string_pretty(&self.metadata)?;
        fs::write(metadata_path, metadata_content)?;

        // Save content files
        for (file_path, content) in &self.content {
            let full_path = path.join(file_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(full_path, content)?;
        }

        Ok(())
    }

    /// Add a file to the package
    pub fn add_file(&mut self, path: String, content: String) {
        self.content.insert(path, content);
    }

    /// Get the main module file (mod.ncl)
    pub fn main_module(&self) -> Option<&String> {
        self.content.get("mod.ncl")
    }

    /// Get all module files
    pub fn modules(&self) -> Vec<&String> {
        self.content.keys().collect()
    }

    /// Validate package structure
    pub fn validate(&self) -> Result<()> {
        // Check for main module
        if !self.content.contains_key("mod.ncl") {
            anyhow::bail!("Package missing main module (mod.ncl)");
        }

        // Validate version format
        semver::Version::parse(&self.metadata.version)?;

        // Validate dependencies
        for dep in &self.metadata.dependencies {
            semver::VersionReq::parse(&dep.version_req)?;
        }

        Ok(())
    }
}

/// Recursively load Nickel files from a directory
fn load_nickel_files(
    base_path: &Path,
    current_path: &Path,
    content: &mut HashMap<String, String>,
) -> Result<()> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            load_nickel_files(base_path, &path, content)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("ncl") {
            let relative_path = path.strip_prefix(base_path)?;
            let file_content = fs::read_to_string(&path)?;
            content.insert(relative_path.to_string_lossy().to_string(), file_content);
        }
    }

    Ok(())
}

/// Package builder for easier construction
pub struct PackageBuilder {
    metadata: PackageMetadata,
    content: HashMap<String, String>,
}

impl PackageBuilder {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            metadata: PackageMetadata {
                name: name.into(),
                version: version.into(),
                description: None,
                categories: Vec::new(),
                keywords: Vec::new(),
                homepage: None,
                repository: None,
                dependencies: Vec::new(),
            },
            content: HashMap::new(),
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.metadata.description = Some(desc.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.metadata.categories.push(category.into());
        self
    }

    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.metadata.keywords.push(keyword.into());
        self
    }

    pub fn dependency(mut self, name: impl Into<String>, version_req: impl Into<String>) -> Self {
        self.metadata.dependencies.push(PackageDependency {
            name: name.into(),
            version_req: version_req.into(),
            optional: false,
            features: Vec::new(),
        });
        self
    }

    pub fn optional_dependency(
        mut self,
        name: impl Into<String>,
        version_req: impl Into<String>,
    ) -> Self {
        self.metadata.dependencies.push(PackageDependency {
            name: name.into(),
            version_req: version_req.into(),
            optional: true,
            features: Vec::new(),
        });
        self
    }

    pub fn file(mut self, path: impl Into<String>, content: impl Into<String>) -> Self {
        self.content.insert(path.into(), content.into());
        self
    }

    pub fn build(self) -> Package {
        Package {
            metadata: self.metadata,
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_builder() {
        let package = PackageBuilder::new("test-pkg", "1.0.0")
            .description("Test package")
            .category("testing")
            .keyword("test")
            .dependency("dep1", "^1.0")
            .file("mod.ncl", "{ test = true }")
            .build();

        assert_eq!(package.metadata.name, "test-pkg");
        assert_eq!(package.metadata.version, "1.0.0");
        assert_eq!(package.metadata.dependencies.len(), 1);
        assert!(package.main_module().is_some());
    }

    #[test]
    fn test_package_validation() {
        let mut package = PackageBuilder::new("test", "1.0.0").build();

        // Should fail without mod.ncl
        assert!(package.validate().is_err());

        // Should pass with mod.ncl
        package.add_file("mod.ncl".to_string(), "{}".to_string());
        assert!(package.validate().is_ok());
    }
}
