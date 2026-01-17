//! Improved Nickel manifest generation using the unified IR pipeline
//!
//! This module provides enhanced functionality to generate Nickel package manifests
//! (Nickel-pkg.ncl files) that properly integrates with the unified IR pipeline.

use crate::CodegenError;
use amalgam_core::ir::{Module, IR};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

/// Enhanced configuration for Nickel package generation
#[derive(Debug, Clone)]
pub struct NickelManifestConfig {
    /// Package name (e.g., "k8s_io", "crossplane")
    pub name: String,
    /// Package version following SemVer
    pub version: String,
    /// Minimum Nickel version required
    pub minimal_nickel_version: String,
    /// Package description
    pub description: String,
    /// Package authors
    pub authors: Vec<String>,
    /// Package license (SPDX identifier)
    pub license: String,
    /// Package keywords for discovery
    pub keywords: Vec<String>,
    /// Base package ID for dependencies (e.g., "github:seryl/nickel-pkgs/pkgs")
    pub base_package_id: Option<String>,
    /// Enable local development mode (use Path dependencies)
    pub local_dev_mode: bool,
    /// Local package path prefix for development
    pub local_package_prefix: Option<String>,
}

impl Default for NickelManifestConfig {
    fn default() -> Self {
        Self {
            name: "generated-types".to_string(),
            version: "0.1.0".to_string(),
            minimal_nickel_version: "1.9.0".to_string(),
            description: "Auto-generated Nickel type definitions".to_string(),
            authors: vec!["amalgam".to_string()],
            license: "Apache-2.0".to_string(),
            keywords: vec!["kubernetes".to_string(), "types".to_string()],
            base_package_id: Some("github:seryl/nickel-pkgs/pkgs".to_string()),
            local_dev_mode: false,
            local_package_prefix: None,
        }
    }
}

/// Dependency specification for Nickel packages
#[derive(Debug, Clone)]
pub enum NickelDependency {
    /// Local path dependency
    Path { path: PathBuf },
    /// Index dependency from a package registry
    Index { package: String, version: String },
    /// Git dependency
    Git {
        url: String,
        branch: Option<String>,
        tag: Option<String>,
        rev: Option<String>,
    },
}

impl NickelDependency {
    /// Convert to Nickel manifest format
    pub fn to_nickel(&self) -> String {
        match self {
            NickelDependency::Path { path } => {
                format!("'Path \"{}\"", path.display())
            }
            NickelDependency::Index { package, version } => {
                format!(
                    "'Index {{ package = \"{}\", version = \"{}\" }}",
                    package, version
                )
            }
            NickelDependency::Git {
                url,
                branch,
                tag,
                rev,
            } => {
                let mut parts = vec![format!("url = \"{}\"", url)];
                if let Some(b) = branch {
                    parts.push(format!("branch = \"{}\"", b));
                }
                if let Some(t) = tag {
                    parts.push(format!("tag = \"{}\"", t));
                }
                if let Some(r) = rev {
                    parts.push(format!("rev = \"{}\"", r));
                }
                format!("'Git {{ {} }}", parts.join(", "))
            }
        }
    }
}

/// Enhanced Nickel manifest generator that works with the unified IR pipeline
pub struct NickelManifestGenerator {
    config: NickelManifestConfig,
}

impl NickelManifestGenerator {
    pub fn new(config: NickelManifestConfig) -> Self {
        Self { config }
    }

    /// Analyze IR to detect required dependencies
    pub fn analyze_dependencies(&self, ir: &IR) -> HashMap<String, NickelDependency> {
        let mut dependencies = HashMap::new();
        let mut has_k8s_refs = false;
        let mut has_crossplane_refs = false;

        // Scan all modules for external type references
        for module in &ir.modules {
            for type_def in &module.types {
                if self.has_reference_to(&type_def.ty, "io.k8s.") {
                    has_k8s_refs = true;
                }
                if self.has_reference_to(&type_def.ty, "apiextensions.crossplane.io") {
                    has_crossplane_refs = true;
                }
            }
        }

        // Add dependencies based on references found
        if has_k8s_refs && !self.config.name.contains("k8s") {
            let dep = if self.config.local_dev_mode {
                let path = self
                    .config
                    .local_package_prefix
                    .as_ref()
                    .map(|p| PathBuf::from(p).join("k8s_io"))
                    .unwrap_or_else(|| PathBuf::from("../k8s_io"));
                NickelDependency::Path { path }
            } else if let Some(base) = &self.config.base_package_id {
                NickelDependency::Index {
                    package: format!("{}/k8s_io", base),
                    version: "0.1.0".to_string(),
                }
            } else {
                NickelDependency::Path {
                    path: PathBuf::from("../k8s_io"),
                }
            };
            dependencies.insert("k8s_io".to_string(), dep);
        }

        if has_crossplane_refs && !self.config.name.contains("crossplane") {
            let dep = if self.config.local_dev_mode {
                let path = self
                    .config
                    .local_package_prefix
                    .as_ref()
                    .map(|p| PathBuf::from(p).join("crossplane"))
                    .unwrap_or_else(|| PathBuf::from("../crossplane"));
                NickelDependency::Path { path }
            } else if let Some(base) = &self.config.base_package_id {
                NickelDependency::Index {
                    package: format!("{}/crossplane", base),
                    version: "0.1.0".to_string(),
                }
            } else {
                NickelDependency::Path {
                    path: PathBuf::from("../crossplane"),
                }
            };
            dependencies.insert("crossplane".to_string(), dep);
        }

        dependencies
    }

    /// Check if a type contains references to a given prefix
    fn has_reference_to(&self, ty: &amalgam_core::types::Type, prefix: &str) -> bool {
        Self::type_has_reference(ty, prefix)
    }

    /// Static helper to check if a type contains references to a given prefix
    fn type_has_reference(ty: &amalgam_core::types::Type, prefix: &str) -> bool {
        use amalgam_core::types::Type;

        match ty {
            Type::Reference { name, .. } if name.contains(prefix) => true,
            Type::Array(inner) | Type::Optional(inner) => Self::type_has_reference(inner, prefix),
            Type::Map { value, .. } => Self::type_has_reference(value, prefix),
            Type::Record { fields, .. } => fields
                .values()
                .any(|f| Self::type_has_reference(&f.ty, prefix)),
            Type::Union { types, .. } => types.iter().any(|t| Self::type_has_reference(t, prefix)),
            Type::TaggedUnion { variants, .. } => variants
                .values()
                .any(|t| Self::type_has_reference(t, prefix)),
            _ => false,
        }
    }

    /// Generate a complete Nickel manifest from IR
    pub fn generate_manifest(
        &self,
        ir: &IR,
        extra_deps: Option<HashMap<String, NickelDependency>>,
    ) -> Result<String, CodegenError> {
        let mut manifest = String::new();

        // Detect dependencies from IR
        let mut dependencies = self.analyze_dependencies(ir);

        // Add any extra dependencies
        if let Some(extra) = extra_deps {
            dependencies.extend(extra);
        }

        // Build the manifest
        manifest.push_str("# Nickel Package Manifest\n");
        manifest.push_str("# Generated by Amalgam using unified IR pipeline\n\n");
        manifest.push_str("{\n");

        // Core metadata
        manifest.push_str(&format!("  name = \"{}\",\n", self.config.name));
        manifest.push_str(&format!("  version = \"{}\",\n", self.config.version));
        manifest.push_str(&format!(
            "  description = \"{}\",\n",
            escape_string(&self.config.description)
        ));

        // Authors
        if !self.config.authors.is_empty() {
            manifest.push_str("  authors = [\n");
            for author in &self.config.authors {
                manifest.push_str(&format!("    \"{}\",\n", escape_string(author)));
            }
            manifest.push_str("  ],\n");
        }

        // License
        manifest.push_str(&format!("  license = \"{}\",\n", self.config.license));

        // Keywords
        if !self.config.keywords.is_empty() {
            manifest.push_str("  keywords = [\n");
            for keyword in &self.config.keywords {
                manifest.push_str(&format!("    \"{}\",\n", escape_string(keyword)));
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

            // Sort dependencies for stable output
            let mut sorted_deps: Vec<_> = dependencies.into_iter().collect();
            sorted_deps.sort_by_key(|(name, _)| name.clone());

            for (name, dep) in sorted_deps {
                manifest.push_str(&format!("    {} = {},\n", name, dep.to_nickel()));
            }
            manifest.push_str("  },\n");
        }

        // Close and apply contract
        manifest.push_str("} | std.package.Manifest\n");

        Ok(manifest)
    }

    /// Generate a main module file that imports all sub-modules
    pub fn generate_main_module(&self, ir: &IR) -> Result<String, CodegenError> {
        let mut main = String::new();

        main.push_str(&format!("# Main module for {}\n", self.config.name));
        main.push_str("# Auto-generated by Amalgam\n\n");
        main.push_str("{\n");

        // Group modules by their namespace
        let mut namespaces: BTreeMap<String, Vec<&Module>> = BTreeMap::new();

        for module in &ir.modules {
            // Extract namespace (e.g., "k8s.io" from "k8s.io.v1.pod")
            let parts: Vec<&str> = module.name.split('.').collect();
            if parts.len() >= 2 {
                let namespace = parts[0..parts.len() - 1].join(".");
                namespaces.entry(namespace).or_default().push(module);
            }
        }

        // Generate imports for each namespace
        for (namespace, modules) in namespaces {
            let safe_name = sanitize_identifier(&namespace);
            main.push_str(&format!("  {} = {{\n", safe_name));

            // Group by version
            let mut versions: BTreeMap<String, Vec<&&Module>> = BTreeMap::new();
            for module in &modules {
                let parts: Vec<&str> = module.name.split('.').collect();
                if let Some(version) = parts.last() {
                    versions
                        .entry(version.to_string())
                        .or_default()
                        .push(module);
                }
            }

            for (version, _) in versions {
                main.push_str(&format!(
                    "    {} = import \"./{}/{}/mod.ncl\",\n",
                    version,
                    namespace.replace('.', "/"),
                    version
                ));
            }

            main.push_str("  },\n");
        }

        main.push_str("}\n");

        Ok(main)
    }
}

/// Sanitize an identifier for use in Nickel
fn sanitize_identifier(s: &str) -> String {
    s.replace(['-', '.'], "_")
}

/// Escape a string for use in Nickel string literals
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use amalgam_core::ir::TypeDefinition;
    use amalgam_core::types::{Field, Type};

    #[test]
    fn test_dependency_detection() {
        let mut ir = IR::new();

        // Add a module with k8s references
        let module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![TypeDefinition {
                name: "TestType".to_string(),
                ty: Type::Record {
                    fields: BTreeMap::from([(
                        "metadata".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_string(),
                                module: None,
                            },
                            required: false,
                            description: None,
                            default: None,
                            validation: None,
                            contracts: Vec::new(),
                        },
                    )]),
                    open: false,
                },
                documentation: None,
                annotations: BTreeMap::new(),
            }],
            constants: vec![],
            metadata: Default::default(),
        };

        ir.add_module(module);

        let config = NickelManifestConfig::default();
        let generator = NickelManifestGenerator::new(config);
        let deps = generator.analyze_dependencies(&ir);

        assert!(deps.contains_key("k8s_io"));
    }

    #[test]
    fn test_manifest_generation() {
        let ir = IR::new();
        let config = NickelManifestConfig {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package with \"quotes\"".to_string(),
            ..Default::default()
        };

        let generator = NickelManifestGenerator::new(config);
        let manifest = generator.generate_manifest(&ir, None).unwrap();

        assert!(manifest.contains("name = \"test-package\""));
        assert!(manifest.contains("version = \"1.0.0\""));
        assert!(manifest.contains("Test package with \\\"quotes\\\""));
        assert!(manifest.ends_with("| std.package.Manifest\n"));
    }
}
