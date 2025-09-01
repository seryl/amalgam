//! Nickel package manifest generation
//!
//! This module provides functionality to generate Nickel package manifests
//! (Nickel-pkg.ncl files) for generated type definitions.

use crate::CodegenError;
use amalgam_core::ir::Module;
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for generating a Nickel package
#[derive(Debug, Clone)]
pub struct NickelPackageConfig {
    /// Package name (e.g., "k8s-types", "crossplane-types")
    pub name: String,
    /// Package version
    pub version: String,
    /// Minimum Nickel version required
    pub minimal_nickel_version: String,
    /// Package description
    pub description: String,
    /// Package authors
    pub authors: Vec<String>,
    /// Package license
    pub license: String,
    /// Package keywords for discovery
    pub keywords: Vec<String>,
}

impl Default for NickelPackageConfig {
    fn default() -> Self {
        Self {
            name: "generated-types".to_string(),
            version: "0.1.0".to_string(),
            minimal_nickel_version: "1.9.0".to_string(),
            description: "Auto-generated Nickel type definitions".to_string(),
            authors: vec!["amalgam".to_string()],
            license: "Apache-2.0".to_string(),
            keywords: vec![
                "kubernetes".to_string(),
                "crd".to_string(),
                "types".to_string(),
            ],
        }
    }
}

/// Generator for Nickel package manifests
pub struct NickelPackageGenerator {
    config: NickelPackageConfig,
}

impl NickelPackageGenerator {
    pub fn new(config: NickelPackageConfig) -> Self {
        Self { config }
    }

    /// Generate a Nickel package manifest for a set of modules
    pub fn generate_manifest(
        &self,
        _modules: &[Module],
        dependencies: HashMap<String, PackageDependency>,
    ) -> Result<String, CodegenError> {
        let mut manifest = String::new();

        // Start the manifest object
        manifest.push_str("{\n");

        // Basic package metadata
        manifest.push_str(&format!("  name = \"{}\",\n", self.config.name));
        manifest.push_str(&format!(
            "  description = \"{}\",\n",
            self.config.description
        ));
        manifest.push_str(&format!("  version = \"{}\",\n", self.config.version));

        // Authors
        if !self.config.authors.is_empty() {
            manifest.push_str("  authors = [\n");
            for author in &self.config.authors {
                manifest.push_str(&format!("    \"{}\",\n", author));
            }
            manifest.push_str("  ],\n");
        }

        // License
        if !self.config.license.is_empty() {
            manifest.push_str(&format!("  license = \"{}\",\n", self.config.license));
        }

        // Keywords
        if !self.config.keywords.is_empty() {
            manifest.push_str("  keywords = [\n");
            for keyword in &self.config.keywords {
                manifest.push_str(&format!("    \"{}\",\n", keyword));
            }
            manifest.push_str("  ],\n");
        }

        // Minimal Nickel version
        manifest.push_str(&format!(
            "  minimal_nickel_version = \"{}\",\n",
            self.config.minimal_nickel_version
        ));

        // Dependencies
        if !dependencies.is_empty() {
            manifest.push_str("  dependencies = {\n");
            for (name, dep) in dependencies {
                manifest.push_str(&format!("    {} = {},\n", name, dep.to_nickel_string()));
            }
            manifest.push_str("  },\n");
        }

        // Close the manifest and apply the contract
        manifest.push_str("} | std.package.Manifest\n");

        Ok(manifest)
    }

    /// Generate a main entry point file that exports all types
    pub fn generate_main_module(&self, modules: &[Module]) -> Result<String, CodegenError> {
        let mut main = String::new();

        main.push_str("# Main module for ");
        main.push_str(&self.config.name);
        main.push('\n');
        main.push_str("# This file exports all generated types\n\n");

        main.push_str("{\n");

        // Group modules by their base name (e.g., group in k8s context)
        let mut grouped_modules: HashMap<String, Vec<&Module>> = HashMap::new();
        for module in modules {
            let parts: Vec<&str> = module.name.split('.').collect();
            if let Some(group) = parts.first() {
                grouped_modules
                    .entry(group.to_string())
                    .or_default()
                    .push(module);
            }
        }

        // Export each group
        for (group, group_modules) in grouped_modules {
            main.push_str(&format!("  {} = {{\n", sanitize_identifier(&group)));

            for module in group_modules {
                // Get the relative module name (e.g., "v1" from "core.v1")
                let parts: Vec<&str> = module.name.split('.').collect();
                if parts.len() > 1 {
                    let version = parts[1];
                    main.push_str(&format!(
                        "    {} = import \"./{}/{}/mod.ncl\",\n",
                        sanitize_identifier(version),
                        group,
                        version
                    ));
                }
            }

            main.push_str("  },\n");
        }

        main.push_str("}\n");

        Ok(main)
    }
}

/// Represents a dependency in a Nickel package
#[derive(Debug, Clone)]
pub enum PackageDependency {
    /// A path dependency to a local package
    Path(PathBuf),
    /// A dependency from the package index
    Index { package: String, version: String },
    /// A git dependency
    Git {
        url: String,
        branch: Option<String>,
        tag: Option<String>,
        rev: Option<String>,
    },
}

impl PackageDependency {
    /// Convert the dependency to its Nickel representation
    pub fn to_nickel_string(&self) -> String {
        match self {
            PackageDependency::Path(path) => {
                format!("'Path \"{}\"", path.display())
            }
            PackageDependency::Index { package, version } => {
                format!(
                    "'Index {{ package = \"{}\", version = \"{}\" }}",
                    package, version
                )
            }
            PackageDependency::Git {
                url,
                branch,
                tag,
                rev,
            } => {
                let mut parts = vec![format!("url = \"{}\"", url)];
                if let Some(branch) = branch {
                    parts.push(format!("branch = \"{}\"", branch));
                }
                if let Some(tag) = tag {
                    parts.push(format!("tag = \"{}\"", tag));
                }
                if let Some(rev) = rev {
                    parts.push(format!("rev = \"{}\"", rev));
                }
                format!("'Git {{ {} }}", parts.join(", "))
            }
        }
    }
}

/// Sanitize an identifier to be valid in Nickel
fn sanitize_identifier(s: &str) -> String {
    // Replace invalid characters with underscores
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_basic_manifest() {
        let config = NickelPackageConfig {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            minimal_nickel_version: "1.9.0".to_string(),
            description: "A test package".to_string(),
            authors: vec!["Test Author".to_string()],
            license: "MIT".to_string(),
            keywords: vec!["test".to_string()],
        };

        let generator = NickelPackageGenerator::new(config);
        let manifest = generator.generate_manifest(&[], HashMap::new()).unwrap();

        assert!(manifest.contains("name = \"test-package\""));
        assert!(manifest.contains("version = \"1.0.0\""));
        assert!(manifest.contains("| std.package.Manifest"));
    }

    #[test]
    fn test_path_dependency() {
        let dep = PackageDependency::Path(PathBuf::from("../k8s-types"));
        assert_eq!(dep.to_nickel_string(), "'Path \"../k8s-types\"");
    }

    #[test]
    fn test_index_dependency() {
        let dep = PackageDependency::Index {
            package: "github:nickel-lang/stdlib".to_string(),
            version: ">=1.0.0".to_string(),
        };
        assert_eq!(
            dep.to_nickel_string(),
            "'Index { package = \"github:nickel-lang/stdlib\", version = \">=1.0.0\" }"
        );
    }
}
