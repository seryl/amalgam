//! Import resolution for cross-package type references

use std::collections::{HashMap, HashSet};
use amalgam_core::types::Type;

/// Represents a type reference that needs to be imported
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeReference {
    /// Group (e.g., "k8s.io", "apiextensions.crossplane.io")
    pub group: String,
    /// Version (e.g., "v1", "v1beta1")
    pub version: String,
    /// Kind (e.g., "ObjectMeta", "Volume")
    pub kind: String,
}

impl TypeReference {
    pub fn new(group: String, version: String, kind: String) -> Self {
        Self { group, version, kind }
    }
    
    /// Parse a fully qualified type reference like "io.k8s.api.core.v1.ObjectMeta"
    pub fn from_qualified_name(name: &str) -> Option<Self> {
        // Handle various formats:
        // - io.k8s.api.core.v1.ObjectMeta
        // - k8s.io/api/core/v1.ObjectMeta
        // - v1.ObjectMeta (assume k8s.io/api/core)
        
        if name.starts_with("io.k8s.") {
            // Handle various k8s formats:
            // - io.k8s.api.core.v1.Pod
            // - io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta
            let parts: Vec<&str> = name.split('.').collect();
            
            if name.starts_with("io.k8s.apimachinery.pkg.apis.meta.") && parts.len() >= 8 {
                // Special case for apimachinery types
                let version = parts[parts.len() - 2].to_string();
                let kind = parts[parts.len() - 1].to_string();
                return Some(Self::new("k8s.io".to_string(), version, kind));
            } else if name.starts_with("io.k8s.api.") && parts.len() >= 5 {
                // Standard API types
                let group = if parts[3] == "core" {
                    "k8s.io".to_string()
                } else {
                    format!("{}.k8s.io", parts[3])
                };
                let version = parts[parts.len() - 2].to_string();
                let kind = parts[parts.len() - 1].to_string();
                return Some(Self::new(group, version, kind));
            }
        } else if name.contains('/') {
            // Format: k8s.io/api/core/v1.ObjectMeta
            let parts: Vec<&str> = name.split('/').collect();
            if let Some(last) = parts.last() {
                let type_parts: Vec<&str> = last.split('.').collect();
                if type_parts.len() == 2 {
                    let version = type_parts[0].to_string();
                    let kind = type_parts[1].to_string();
                    let group = parts[0].to_string();
                    return Some(Self::new(group, version, kind));
                }
            }
        } else if name.starts_with("v1.") || name.starts_with("v1beta1.") || name.starts_with("v1alpha1.") {
            // Short format: v1.ObjectMeta (assume core k8s types)
            let parts: Vec<&str> = name.split('.').collect();
            if parts.len() == 2 {
                return Some(Self::new(
                    "k8s.io".to_string(),
                    parts[0].to_string(),
                    parts[1].to_string(),
                ));
            }
        }
        
        None
    }
    
    /// Get the import path for this reference relative to a base path
    pub fn import_path(&self, _from_group: &str, _from_version: &str) -> String {
        // Calculate relative path from current location to referenced type
        let target_path = format!("{}/{}/{}.ncl", 
            self.group.replace('.', "_"), 
            self.version, 
            self.kind.to_lowercase()
        );
        
        // We're in a file at: group/version/kind.ncl
        // We need to go up 2 levels to get to package root (version -> group -> package root)
        let up_dirs = "../../";
        
        format!("{}{}", up_dirs, target_path)
    }
    
    /// Get the module alias for imports
    pub fn module_alias(&self) -> String {
        format!("{}_{}", 
            self.group.replace('.', "_").replace('-', "_"),
            self.version.replace('-', "_")
        )
    }
}

/// Analyzes types to find external references that need imports
pub struct ImportResolver {
    /// Set of all type references found
    references: HashSet<TypeReference>,
    /// Known types that are already defined locally
    local_types: HashSet<String>,
}

impl ImportResolver {
    pub fn new() -> Self {
        Self {
            references: HashSet::new(),
            local_types: HashSet::new(),
        }
    }
    
    /// Add a locally defined type
    pub fn add_local_type(&mut self, name: &str) {
        self.local_types.insert(name.to_string());
    }
    
    /// Analyze a type and collect external references
    pub fn analyze_type(&mut self, ty: &Type) {
        match ty {
            Type::Reference(name) => {
                // Check if this is an external reference
                if !self.local_types.contains(name) {
                    if let Some(type_ref) = TypeReference::from_qualified_name(name) {
                        tracing::trace!("ImportResolver: found external reference: {:?}", type_ref);
                        self.references.insert(type_ref);
                    } else {
                        tracing::trace!("ImportResolver: could not parse reference: {}", name);
                    }
                }
            }
            Type::Array(inner) => self.analyze_type(inner),
            Type::Optional(inner) => self.analyze_type(inner),
            Type::Map { value, .. } => self.analyze_type(value),
            Type::Record { fields, .. } => {
                for field in fields.values() {
                    self.analyze_type(&field.ty);
                }
            }
            Type::Union(types) => {
                for ty in types {
                    self.analyze_type(ty);
                }
            }
            Type::TaggedUnion { variants, .. } => {
                for ty in variants.values() {
                    self.analyze_type(ty);
                }
            }
            Type::Contract { base, .. } => self.analyze_type(base),
            _ => {}
        }
    }
    
    /// Get all collected references
    pub fn references(&self) -> &HashSet<TypeReference> {
        &self.references
    }
    
    /// Generate import statements for Nickel
    pub fn generate_imports(&self, from_group: &str, from_version: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        // Group references by their module
        let mut by_module: HashMap<String, Vec<&TypeReference>> = HashMap::new();
        for type_ref in &self.references {
            let module_key = format!("{}/{}", type_ref.group, type_ref.version);
            by_module.entry(module_key).or_default().push(type_ref);
        }
        
        // Generate import statements
        for (_module, refs) in by_module {
            let first_ref = refs[0];
            let import_path = first_ref.import_path(from_group, from_version);
            let alias = first_ref.module_alias();
            
            imports.push(format!("let {} = import \"{}\" in", alias, import_path));
        }
        
        imports.sort();
        imports
    }
}

/// Common Kubernetes types that are frequently referenced
pub fn common_k8s_types() -> Vec<TypeReference> {
    vec![
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "ObjectMeta".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "ListMeta".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "TypeMeta".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "LabelSelector".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "Volume".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "VolumeMount".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "Container".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "PodSpec".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "ResourceRequirements".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "Affinity".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "Toleration".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "EnvVar".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "ConfigMapKeySelector".to_string()),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "SecretKeySelector".to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_qualified_name() {
        let ref1 = TypeReference::from_qualified_name("io.k8s.api.core.v1.ObjectMeta");
        assert!(ref1.is_some());
        let ref1 = ref1.unwrap();
        assert_eq!(ref1.group, "k8s.io");
        assert_eq!(ref1.version, "v1");
        assert_eq!(ref1.kind, "ObjectMeta");
        
        let ref2 = TypeReference::from_qualified_name("v1.Volume");
        assert!(ref2.is_some());
        let ref2 = ref2.unwrap();
        assert_eq!(ref2.group, "k8s.io");
        assert_eq!(ref2.version, "v1");
        assert_eq!(ref2.kind, "Volume");
    }
    
    #[test]
    fn test_import_path() {
        let type_ref = TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "ObjectMeta".to_string(),
        );
        
        let path = type_ref.import_path("apiextensions.crossplane.io", "v1");
        assert_eq!(path, "../../k8s_io/v1/objectmeta.ncl");
    }
}