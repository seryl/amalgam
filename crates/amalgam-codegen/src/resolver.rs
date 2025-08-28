//! Extensible type reference resolution system
//! 
//! Based on compiler design principles for name resolution with pluggable strategies.
//! New resolution strategies can be added without modifying existing code.

use std::collections::HashMap;
use amalgam_core::ir::{Module, Import};

/// Result of attempting to resolve a type reference
#[derive(Debug, Clone)]
pub struct Resolution {
    /// The resolved reference to use in generated code
    pub resolved_name: String,
    /// The import that provides this type (if any)
    pub required_import: Option<Import>,
}

/// Trait for implementing type resolution strategies
/// 
/// Each implementation handles a specific pattern of imports/references
/// (e.g., Kubernetes, Crossplane, custom CRDs, etc.)
pub trait ReferenceResolver: Send + Sync {
    /// Check if this resolver can handle the given reference
    fn can_resolve(&self, reference: &str) -> bool;
    
    /// Try to resolve a type reference given the current imports
    fn resolve(
        &self,
        reference: &str,
        imports: &[Import],
        context: &ResolutionContext,
    ) -> Option<Resolution>;
    
    /// Extract type information from an import path
    /// Returns (group, version, kind) if applicable
    fn parse_import_path(&self, path: &str) -> Option<ImportMetadata>;
    
    /// Get a human-readable name for this resolver (for debugging)
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct ImportMetadata {
    pub group: String,
    pub version: String,
    pub kind: Option<String>,
    pub is_module: bool,
}

#[derive(Debug, Clone)]
pub struct ResolutionContext {
    /// Current module's group (e.g., "apiextensions.crossplane.io")
    pub current_group: Option<String>,
    /// Current module's version (e.g., "v1beta1")
    pub current_version: Option<String>,
    /// Current module's kind (e.g., "composition")
    pub current_kind: Option<String>,
}

/// Main resolver that delegates to registered strategies
pub struct TypeResolver {
    /// Registered resolution strategies
    resolvers: Vec<Box<dyn ReferenceResolver>>,
    /// Cache of resolved references for performance
    cache: HashMap<String, Resolution>,
}

impl TypeResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            resolvers: Vec::new(),
            cache: HashMap::new(),
        };
        
        // Register default resolvers
        resolver.register(Box::new(KubernetesResolver::new()));
        resolver.register(Box::new(LocalTypeResolver::new()));
        // More resolvers can be added here as they're implemented
        
        resolver
    }
    
    /// Register a new resolution strategy
    pub fn register(&mut self, resolver: Box<dyn ReferenceResolver>) {
        self.resolvers.push(resolver);
    }
    
    /// Resolve a type reference using registered strategies
    pub fn resolve(
        &mut self,
        reference: &str,
        module: &Module,
        context: &ResolutionContext,
    ) -> String {
        // Check cache first
        if let Some(cached) = self.cache.get(reference) {
            tracing::trace!("TypeResolver: cache hit for '{}'", reference);
            return cached.resolved_name.clone();
        }
        
        tracing::trace!("TypeResolver: resolving '{}' with {} imports", reference, module.imports.len());
        
        // Try each resolver in order
        for resolver in &self.resolvers {
            if resolver.can_resolve(reference) {
                tracing::trace!("  Trying resolver: {}", resolver.name());
                if let Some(resolution) = resolver.resolve(reference, &module.imports, context) {
                    tracing::debug!("TypeResolver: resolved '{}' -> '{}'", reference, resolution.resolved_name);
                    self.cache.insert(reference.to_string(), resolution.clone());
                    return resolution.resolved_name;
                }
            }
        }
        
        tracing::trace!("TypeResolver: no resolver handled '{}', returning as-is", reference);
        // No resolver could handle it - return as-is
        reference.to_string()
    }
    
    /// Clear the resolution cache (useful when context changes)
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

// ============================================================================
// Kubernetes Resolution Strategy
// ============================================================================

struct KubernetesResolver {
    /// Known k8s type mappings
    known_types: HashMap<String, String>,
}

impl KubernetesResolver {
    fn new() -> Self {
        let mut known_types = HashMap::new();
        
        // Register common k8s types and their canonical names
        known_types.insert("ObjectMeta".to_string(), "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_string());
        known_types.insert("ListMeta".to_string(), "io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta".to_string());
        known_types.insert("TypeMeta".to_string(), "io.k8s.apimachinery.pkg.apis.meta.v1.TypeMeta".to_string());
        known_types.insert("LabelSelector".to_string(), "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector".to_string());
        // Add more as needed
        
        Self { known_types }
    }
}

impl ReferenceResolver for KubernetesResolver {
    fn can_resolve(&self, reference: &str) -> bool {
        reference.starts_with("io.k8s.") 
            || reference.contains("k8s.io")
            || self.known_types.contains_key(reference)
    }
    
    fn resolve(
        &self,
        reference: &str,
        imports: &[Import],
        _context: &ResolutionContext,
    ) -> Option<Resolution> {
        // First, normalize the reference if it's a short name
        let full_reference = if let Some(full_name) = self.known_types.get(reference) {
            full_name.clone()
        } else {
            reference.to_string()
        };
        
        tracing::trace!("KubernetesResolver: resolving '{}' (full: '{}')", reference, full_reference);
        
        // Look for a matching import
        for import in imports {
            tracing::trace!("  Checking import: path='{}', alias={:?}", import.path, import.alias);
            if let Some(metadata) = self.parse_import_path(&import.path) {
                tracing::trace!("    Parsed metadata: group={}, version={}, kind={:?}", 
                         metadata.group, metadata.version, metadata.kind);
                // Check if this import could provide the type
                if self.import_provides_type(&metadata, &full_reference) {
                    let alias = import.alias.as_ref().unwrap_or(&metadata.group);
                    let type_name = full_reference.split('.').last().unwrap_or(&full_reference);
                    
                    tracing::debug!("    Resolved '{}' to '{}.{}'", reference, alias, type_name);
                    return Some(Resolution {
                        resolved_name: format!("{}.{}", alias, type_name),
                        required_import: Some(import.clone()),
                    });
                } else {
                    tracing::trace!("    No match (import_provides_type returned false)");
                }
            } else {
                tracing::trace!("    Could not parse import path");
            }
        }
        
        None
    }
    
    fn parse_import_path(&self, path: &str) -> Option<ImportMetadata> {
        // Parse k8s import paths like "../../k8s_io/v1/objectmeta.ncl"
        if !path.contains("k8s_io") && !path.contains("k8s.io") {
            return None;
        }
        
        let parts: Vec<&str> = path.split('/').collect();
        
        // Find k8s_io in the path
        if let Some(k8s_idx) = parts.iter().position(|&p| p == "k8s_io" || p == "k8s.io") {
            // For k8s_io paths, structure is: k8s_io/version/kind.ncl
            if k8s_idx + 2 < parts.len() {
                let version = parts[k8s_idx + 1].to_string();
                let filename = parts[k8s_idx + 2];
                
                let (kind, is_module) = if filename == "mod.ncl" {
                    (None, true)
                } else {
                    let kind_name = filename.strip_suffix(".ncl")?;
                    (Some(capitalize_first(kind_name)), false)
                };
                
                return Some(ImportMetadata {
                    group: "k8s.io".to_string(),  // Simplified - k8s_io maps to k8s.io
                    version,
                    kind,
                    is_module,
                });
            }
        }
        
        None
    }
    
    fn name(&self) -> &str {
        "KubernetesResolver"
    }
}

impl KubernetesResolver {
    fn import_provides_type(&self, metadata: &ImportMetadata, reference: &str) -> bool {
        // Check if the import metadata matches the reference
        if let Some(ref kind) = metadata.kind {
            // Case-insensitive comparison for the kind name
            // The reference might have "ObjectMeta" while the file is "objectmeta.ncl"
            let ref_kind = reference.split('.').last().unwrap_or("");
            ref_kind.eq_ignore_ascii_case(kind)
        } else {
            // Module import - check if version matches
            reference.contains(&metadata.version)
        }
    }
}

// ============================================================================
// Local Type Resolution Strategy
// ============================================================================

struct LocalTypeResolver;

impl LocalTypeResolver {
    fn new() -> Self {
        Self
    }
}

impl ReferenceResolver for LocalTypeResolver {
    fn can_resolve(&self, reference: &str) -> bool {
        // Handle simple, unqualified type names that might be local
        !reference.contains('.') && !reference.contains('/')
    }
    
    fn resolve(
        &self,
        reference: &str,
        _imports: &[Import],
        _context: &ResolutionContext,
    ) -> Option<Resolution> {
        // Local types are used as-is
        Some(Resolution {
            resolved_name: reference.to_string(),
            required_import: None,
        })
    }
    
    fn parse_import_path(&self, _path: &str) -> Option<ImportMetadata> {
        None // Local resolver doesn't parse imports
    }
    
    fn name(&self) -> &str {
        "LocalTypeResolver"
    }
}

// ============================================================================
// Future resolvers can be added here:
// - CrossplaneResolver
// - OpenAPIResolver
// - CustomCRDResolver
// - ProtoResolver
// etc.
// ============================================================================

/// Helper function to capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

impl Default for TypeResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kubernetes_resolution() {
        let mut resolver = TypeResolver::new();
        let module = Module {
            name: "test".to_string(),
            imports: vec![Import {
                path: "../../../k8s.io/apimachinery/v1/mod.ncl".to_string(),
                alias: Some("k8s_v1".to_string()),
                items: vec![],
            }],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };
        
        let context = ResolutionContext {
            current_group: None,
            current_version: None,
            current_kind: None,
        };
        
        let resolved = resolver.resolve("io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta", &module, &context);
        assert_eq!(resolved, "k8s_v1.ObjectMeta");
    }
    
    #[test]
    fn test_local_type_resolution() {
        let mut resolver = TypeResolver::new();
        let module = Module {
            name: "test".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };
        
        let context = ResolutionContext {
            current_group: None,
            current_version: None,
            current_kind: None,
        };
        
        let resolved = resolver.resolve("MyLocalType", &module, &context);
        assert_eq!(resolved, "MyLocalType");
    }
}