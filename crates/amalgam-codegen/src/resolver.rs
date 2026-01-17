//! Generic type reference resolution system
//!
//! This resolver doesn't special-case any particular schema source.
//! It works by matching type references to imports based on configurable patterns.

use amalgam_core::ir::{Import, Module};
use std::collections::HashMap;

/// Result of attempting to resolve a type reference
#[derive(Debug, Clone)]
pub struct Resolution {
    /// The resolved reference to use in generated code
    pub resolved_name: String,
    /// The import that provides this type (if any)
    pub required_import: Option<Import>,
}

/// Context for type resolution
#[derive(Debug, Clone, Default)]
pub struct ResolutionContext {
    pub current_group: Option<String>,
    pub current_version: Option<String>,
    pub current_kind: Option<String>,
}

/// Main type resolver that coordinates resolution strategies
pub struct TypeResolver {
    /// Cache of resolved references
    cache: HashMap<String, Resolution>,
    /// Known type mappings (short name -> full name)
    type_registry: HashMap<String, String>,
}

impl Default for TypeResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            cache: HashMap::new(),
            type_registry: HashMap::new(),
        };

        // Register common type mappings
        resolver.register_common_types();
        resolver
    }

    fn register_common_types(&mut self) {
        // Kubernetes common types
        self.type_registry.insert(
            "ObjectMeta".to_string(),
            "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_string(),
        );
        self.type_registry.insert(
            "LabelSelector".to_string(),
            "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector".to_string(),
        );
        self.type_registry.insert(
            "Time".to_string(),
            "io.k8s.apimachinery.pkg.apis.meta.v1.Time".to_string(),
        );

        // Can be extended with more mappings as needed
    }

    /// Resolve a type reference to its qualified name
    pub fn resolve(
        &mut self,
        reference: &str,
        module: &Module,
        _context: &ResolutionContext,
    ) -> String {
        // Check cache first
        if let Some(cached) = self.cache.get(reference) {
            return cached.resolved_name.clone();
        }

        // Expand short names to full names if known
        let full_reference = self
            .type_registry
            .get(reference)
            .cloned()
            .unwrap_or_else(|| reference.to_string());

        // Try to match against imports
        for import in &module.imports {
            if let Some(resolved) = self.try_resolve_with_import(&full_reference, import) {
                self.cache.insert(
                    reference.to_string(),
                    Resolution {
                        resolved_name: resolved.clone(),
                        required_import: Some(import.clone()),
                    },
                );
                return resolved;
            }
        }

        // Check if it's a local type (defined in current module)
        for type_def in &module.types {
            if type_def.name == reference {
                let resolution = Resolution {
                    resolved_name: reference.to_string(),
                    required_import: None,
                };
                self.cache.insert(reference.to_string(), resolution.clone());
                return resolution.resolved_name;
            }
        }

        // If no resolution found, validate the reference is a valid Nickel identifier
        // If it contains dots, slashes, or other special characters, it would be invalid
        // Fall back to Dyn instead of generating broken code
        if reference.contains('.') || reference.contains('/') || reference.contains(':') {
            tracing::warn!(
                "Cannot resolve type reference '{}' in module '{}', using Dyn fallback",
                reference,
                module.name
            );
            return "Dyn".to_string();
        }

        // Simple identifier - return as-is
        reference.to_string()
    }

    /// Try to resolve a reference using a specific import
    fn try_resolve_with_import(&self, reference: &str, import: &Import) -> Option<String> {
        // Extract the type name from the reference
        // For "apiextensions.crossplane.io/v1/Composition", we want "Composition"
        let type_name = reference.split('/').next_back()?.split('.').next_back()?;

        // Parse the import path to understand what it provides
        let import_info = self.parse_import_path(&import.path)?;

        // Check if this import could provide the requested type
        if self.import_matches_reference(&import_info, reference) {
            // Use the import alias if provided, otherwise use a derived name
            let prefix = import.alias.as_ref().unwrap_or(&import_info.module_name);

            // Check if this is a specific type file import with explicit items list
            // In this case, the imported alias IS the type, not a module containing types
            let ref_type_name = reference
                .split('/')
                .next_back()
                .unwrap_or(reference)
                .split('.')
                .next_back()
                .unwrap_or(reference);
            let import_type_name = import_info.module_name.to_lowercase();

            if ref_type_name.to_lowercase() == import_type_name
                && import_info.module_name != "mod"
                && !import.items.is_empty()
            {
                // This is a specific type file with explicit items - the alias IS the type
                return Some(prefix.to_string());
            } else {
                // This is a module import or pattern-matched import - use alias.TypeName format
                return Some(format!("{}.{}", prefix, type_name));
            }
        }

        None
    }

    /// Parse an import path to extract metadata
    fn parse_import_path(&self, path: &str) -> Option<ImportInfo> {
        // Remove .ncl extension if present
        let path = path.trim_end_matches(".ncl");

        // Split into components
        let parts: Vec<&str> = path.split('/').collect();
        if parts.is_empty() {
            return None;
        }

        // Filter out relative path components
        let clean_parts: Vec<&str> = parts
            .iter()
            .filter(|&&p| !p.is_empty() && p != ".." && p != ".")
            .cloned()
            .collect();

        if clean_parts.is_empty() {
            return None;
        }

        // Get the last component as the module name
        let module_name = clean_parts.last()?.to_string();

        // Extract namespace from the clean path (everything except filename)
        let namespace = if clean_parts.len() > 1 {
            clean_parts[..clean_parts.len() - 1].join(".")
        } else {
            String::new()
        };

        Some(ImportInfo {
            module_name: module_name.to_string(),
            namespace,
            full_path: path.to_string(),
        })
    }

    /// Check if an import can provide a specific type reference
    fn import_matches_reference(&self, import_info: &ImportInfo, reference: &str) -> bool {
        // More precise matching: check if this import provides the specific type we need
        // Extract the type name from the reference
        // For "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta", we want "ObjectMeta"
        // For "apiextensions.crossplane.io/v1/Composition", we want "Composition"
        let ref_type_name = reference
            .split('/')
            .next_back()
            .unwrap_or(reference)
            .split('.')
            .next_back()
            .unwrap_or(reference);

        // Extract the type name from the import module name (e.g., "objectmeta" from module_name)
        // The module_name is typically the filename without extension (e.g., "objectmeta", "volume")
        let import_type_name = import_info.module_name.to_lowercase();

        // Check if this is a specific type file import (e.g., "objectmeta.ncl")
        // This handles the case where import path is like "../../../k8s_io/v1/objectmeta.ncl"
        // and we're looking for "ObjectMeta"
        if ref_type_name.to_lowercase() == import_type_name {
            return true;
        }

        // Check if this is a module import (e.g., "mod.ncl") that could provide many types
        // In this case, check if the namespace/version matches
        if import_info.module_name == "mod" && !import_info.namespace.is_empty() {
            // For module imports, we need to check if this module could provide the requested type
            // For k8s imports like "k8s.io/apimachinery/v1/mod.ncl", it should match types from
            // "io.k8s.apimachinery.pkg.apis.meta.v1.*"

            // Check if the reference contains key identifying parts of the namespace
            // For example, "apimachinery" and "v1" from the import should appear in the reference
            let namespace_parts: Vec<&str> = import_info.namespace.split('.').collect();

            // For k8s specifically, check for the pattern
            if namespace_parts.contains(&"k8s") && reference.contains("io.k8s.") {
                // Check if this is the right k8s module by looking at version and API group
                if let Some(version_idx) = namespace_parts.iter().position(|&p| p.starts_with('v'))
                {
                    let version = namespace_parts[version_idx];
                    if reference.contains(version) {
                        // Also check API group if present
                        if namespace_parts.contains(&"apimachinery") {
                            return reference.contains("apimachinery");
                        } else if namespace_parts.contains(&"api") {
                            return reference.contains("api.core")
                                || reference.contains("api.apps");
                        }
                        return true;
                    }
                }
            }

            // For non-k8s module imports, use simpler matching
            return namespace_parts
                .iter()
                .filter(|&&p| p.len() > 2)
                .all(|&part| reference.contains(part));
        }

        false
    }
}

#[derive(Debug)]
struct ImportInfo {
    module_name: String,
    #[allow(dead_code)]
    namespace: String,
    #[allow(dead_code)]
    full_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use amalgam_core::ir::Metadata;
    use std::collections::BTreeMap;

    fn create_test_module(name: &str, imports: Vec<Import>) -> Module {
        Module {
            name: name.to_string(),
            imports,
            types: vec![],
            constants: vec![],
            metadata: Metadata {
                source_language: None,
                source_file: None,
                version: None,
                generated_at: None,
                custom: BTreeMap::new(),
            },
        }
    }

    #[test]
    fn test_kubernetes_resolution() {
        let mut resolver = TypeResolver::new();
        let module = create_test_module(
            "test",
            vec![Import {
                path: "../../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("objectmeta".to_string()),
                items: vec!["ObjectMeta".to_string()],
            }],
        );

        let resolved = resolver.resolve(
            "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
            &module,
            &ResolutionContext::default(),
        );
        assert_eq!(resolved, "objectmeta");
    }

    #[test]
    fn test_short_name_resolution() {
        let mut resolver = TypeResolver::new();
        let module = create_test_module(
            "test",
            vec![Import {
                path: "../../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("objectmeta".to_string()),
                items: vec!["ObjectMeta".to_string()],
            }],
        );

        // Should expand ObjectMeta to full name and resolve
        let resolved = resolver.resolve("ObjectMeta", &module, &ResolutionContext::default());
        assert_eq!(resolved, "objectmeta");
    }

    #[test]
    fn test_local_type_resolution() {
        let mut resolver = TypeResolver::new();
        let mut module = create_test_module("test", vec![]);

        // Add a local type
        module.types.push(amalgam_core::ir::TypeDefinition {
            name: "MyType".to_string(),
            ty: amalgam_core::types::Type::String,
            documentation: None,
            annotations: BTreeMap::new(),
        });

        let resolved = resolver.resolve("MyType", &module, &ResolutionContext::default());
        assert_eq!(resolved, "MyType");
    }

    #[test]
    fn test_crossplane_resolution() {
        let mut resolver = TypeResolver::new();
        let module = create_test_module(
            "test",
            vec![Import {
                path: "../../apiextensions.crossplane.io/v1/composition.ncl".to_string(),
                alias: Some("crossplane_v1".to_string()),
                items: vec![],
            }],
        );

        let resolved = resolver.resolve(
            "apiextensions.crossplane.io/v1/Composition",
            &module,
            &ResolutionContext::default(),
        );

        // The resolver sees "v1" in both the import path and reference, so it matches
        assert!(resolved.ends_with("Composition"));
        assert!(resolved.contains("crossplane"));
    }

    #[test]
    fn test_unresolved_type() {
        let mut resolver = TypeResolver::new();
        let module = create_test_module("test", vec![]);

        // Unknown type should be returned as-is
        let resolved = resolver.resolve("UnknownType", &module, &ResolutionContext::default());
        assert_eq!(resolved, "UnknownType");
    }
}
