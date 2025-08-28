//! Authoritative Kubernetes type definitions from Go source

use crate::{
    go_ast::{GoASTParser, GoTypeInfo},
    imports::TypeReference,
    ParserError,
};
use amalgam_core::{ir::TypeDefinition, types::Type};
use std::collections::{BTreeMap, HashMap};

/// Authoritative source for Kubernetes type definitions
pub struct K8sAuthoritativeTypes {
    /// Parsed Go type information
    go_types: HashMap<String, GoTypeInfo>,
    /// Mapping from Go qualified names to TypeReferences
    type_mapping: HashMap<String, TypeReference>,
}

impl Default for K8sAuthoritativeTypes {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sAuthoritativeTypes {
    pub fn new() -> Self {
        Self {
            go_types: HashMap::new(),
            type_mapping: HashMap::new(),
        }
    }

    /// Initialize with authoritative Kubernetes types from Go source
    pub async fn initialize(&mut self) -> Result<(), ParserError> {
        let mut parser = GoASTParser::new();

        // Parse core Kubernetes types
        let k8s_types = parser.parse_k8s_core_types().await?;
        self.go_types = k8s_types;

        // Build the type mapping
        self.build_type_mapping()?;

        Ok(())
    }

    /// Build mapping from Go qualified names to TypeReferences
    fn build_type_mapping(&mut self) -> Result<(), ParserError> {
        for (qualified_name, type_info) in &self.go_types {
            if let Some(type_ref) = self.go_qualified_name_to_type_ref(qualified_name, type_info) {
                self.type_mapping.insert(qualified_name.clone(), type_ref);
            }
        }
        Ok(())
    }

    /// Convert Go qualified name and type info to TypeReference
    fn go_qualified_name_to_type_ref(
        &self,
        _qualified_name: &str,
        type_info: &GoTypeInfo,
    ) -> Option<TypeReference> {
        // Parse package path to determine group and version
        // Examples:
        // - k8s.io/api/core/v1.ObjectMeta -> k8s.io, v1, ObjectMeta
        // - k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta -> k8s.io, v1, ObjectMeta

        if type_info.package_path.starts_with("k8s.io/api/core/") {
            let version = type_info.package_path.strip_prefix("k8s.io/api/core/")?;
            Some(TypeReference::new(
                "k8s.io".to_string(),
                version.to_string(),
                type_info.name.clone(),
            ))
        } else if type_info
            .package_path
            .starts_with("k8s.io/apimachinery/pkg/apis/meta/")
        {
            let version = type_info
                .package_path
                .strip_prefix("k8s.io/apimachinery/pkg/apis/meta/")?;
            Some(TypeReference::new(
                "k8s.io".to_string(),
                version.to_string(),
                type_info.name.clone(),
            ))
        } else if type_info.package_path.starts_with("k8s.io/api/apps/") {
            let version = type_info.package_path.strip_prefix("k8s.io/api/apps/")?;
            Some(TypeReference::new(
                "apps.k8s.io".to_string(),
                version.to_string(),
                type_info.name.clone(),
            ))
        } else if type_info.package_path.starts_with("k8s.io/api/networking/") {
            let version = type_info
                .package_path
                .strip_prefix("k8s.io/api/networking/")?;
            Some(TypeReference::new(
                "networking.k8s.io".to_string(),
                version.to_string(),
                type_info.name.clone(),
            ))
        } else {
            None
        }
    }

    /// Get authoritative type information for a Go type
    pub fn get_go_type(&self, qualified_name: &str) -> Option<&GoTypeInfo> {
        self.go_types.get(qualified_name)
    }

    /// Get TypeReference for a Go qualified name
    pub fn get_type_reference(&self, qualified_name: &str) -> Option<&TypeReference> {
        self.type_mapping.get(qualified_name)
    }

    /// Convert Go type to Nickel TypeDefinition using authoritative data
    pub fn go_type_to_nickel_definition(
        &self,
        go_type: &GoTypeInfo,
    ) -> Result<TypeDefinition, ParserError> {
        let parser = GoASTParser::new();
        let nickel_type = parser.go_type_to_nickel(go_type)?;

        Ok(TypeDefinition {
            name: go_type.name.clone(),
            ty: nickel_type,
            documentation: go_type.documentation.clone(),
            annotations: BTreeMap::new(),
        })
    }

    /// Check if a field name in a CRD schema should be replaced with a known k8s type
    pub fn should_replace_field(&self, field_name: &str, current_type: &Type) -> Option<String> {
        match field_name {
            "metadata" if matches!(current_type, Type::Record { fields, .. } if fields.is_empty()) => {
                Some("k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta".to_string())
            }
            "status" => {
                // Status fields could reference specific status types
                None // For now, leave as-is
            }
            _ => None,
        }
    }

    /// Get field replacements based on field patterns and context
    pub fn get_field_replacements(
        &self,
        field_name: &str,
        current_type: &Type,
        parent_context: Option<&str>,
    ) -> Option<String> {
        match (field_name, parent_context) {
            // Core metadata
            ("metadata", _) if matches!(current_type, Type::Record { fields, .. } if fields.is_empty()) => {
                Some("k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta".to_string())
            }

            // Common volume patterns
            ("volumes", Some("spec")) if matches!(current_type, Type::Array(_)) => {
                Some("[]k8s.io/api/core/v1.Volume".to_string())
            }
            ("volumeMounts", _) if matches!(current_type, Type::Array(_)) => {
                Some("[]k8s.io/api/core/v1.VolumeMount".to_string())
            }

            // Container patterns
            ("containers", Some("spec")) if matches!(current_type, Type::Array(_)) => {
                Some("[]k8s.io/api/core/v1.Container".to_string())
            }
            ("initContainers", Some("spec")) if matches!(current_type, Type::Array(_)) => {
                Some("[]k8s.io/api/core/v1.Container".to_string())
            }

            // Resource patterns
            ("resources", _) if matches!(current_type, Type::Record { .. }) => {
                Some("k8s.io/api/core/v1.ResourceRequirements".to_string())
            }

            // Selector patterns
            ("selector", _) if matches!(current_type, Type::Record { .. }) => {
                Some("k8s.io/apimachinery/pkg/apis/meta/v1.LabelSelector".to_string())
            }

            // Environment variables
            ("env", _) if matches!(current_type, Type::Array(_)) => {
                Some("[]k8s.io/api/core/v1.EnvVar".to_string())
            }
            ("envFrom", _) if matches!(current_type, Type::Array(_)) => {
                Some("[]k8s.io/api/core/v1.EnvFromSource".to_string())
            }

            // Affinity and scheduling
            ("affinity", _) if matches!(current_type, Type::Record { .. }) => {
                Some("k8s.io/api/core/v1.Affinity".to_string())
            }
            ("tolerations", _) if matches!(current_type, Type::Array(_)) => {
                Some("[]k8s.io/api/core/v1.Toleration".to_string())
            }
            ("nodeSelector", _) if matches!(current_type, Type::Map { .. }) => {
                Some("map[string]string".to_string()) // Keep as map, but with precise typing
            }

            // Security context
            ("securityContext", _) if matches!(current_type, Type::Record { .. }) => {
                Some("k8s.io/api/core/v1.SecurityContext".to_string())
            }
            ("podSecurityContext", _) if matches!(current_type, Type::Record { .. }) => {
                Some("k8s.io/api/core/v1.PodSecurityContext".to_string())
            }

            _ => None,
        }
    }
}

/// Pre-built registry of common Kubernetes type patterns
pub struct K8sTypePatterns {
    patterns: HashMap<String, String>,
}

impl Default for K8sTypePatterns {
    fn default() -> Self {
        Self::new()
    }
}

impl K8sTypePatterns {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Add common field -> Go type mappings
        patterns.insert(
            "metadata".to_string(),
            "k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta".to_string(),
        );
        patterns.insert(
            "spec.volumes".to_string(),
            "[]k8s.io/api/core/v1.Volume".to_string(),
        );
        patterns.insert(
            "spec.containers".to_string(),
            "[]k8s.io/api/core/v1.Container".to_string(),
        );
        patterns.insert(
            "spec.initContainers".to_string(),
            "[]k8s.io/api/core/v1.Container".to_string(),
        );
        patterns.insert(
            "spec.template.spec.volumes".to_string(),
            "[]k8s.io/api/core/v1.Volume".to_string(),
        );
        patterns.insert(
            "spec.template.spec.containers".to_string(),
            "[]k8s.io/api/core/v1.Container".to_string(),
        );
        patterns.insert(
            "spec.selector".to_string(),
            "k8s.io/apimachinery/pkg/apis/meta/v1.LabelSelector".to_string(),
        );
        patterns.insert(
            "spec.template.metadata".to_string(),
            "k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta".to_string(),
        );

        // Resource requirements
        patterns.insert(
            "resources".to_string(),
            "k8s.io/api/core/v1.ResourceRequirements".to_string(),
        );
        patterns.insert(
            "spec.resources".to_string(),
            "k8s.io/api/core/v1.ResourceRequirements".to_string(),
        );

        // Environment variables
        patterns.insert("env".to_string(), "[]k8s.io/api/core/v1.EnvVar".to_string());
        patterns.insert(
            "envFrom".to_string(),
            "[]k8s.io/api/core/v1.EnvFromSource".to_string(),
        );

        // Volume mounts
        patterns.insert(
            "volumeMounts".to_string(),
            "[]k8s.io/api/core/v1.VolumeMount".to_string(),
        );

        // Security and scheduling
        patterns.insert(
            "securityContext".to_string(),
            "k8s.io/api/core/v1.SecurityContext".to_string(),
        );
        patterns.insert(
            "affinity".to_string(),
            "k8s.io/api/core/v1.Affinity".to_string(),
        );
        patterns.insert(
            "tolerations".to_string(),
            "[]k8s.io/api/core/v1.Toleration".to_string(),
        );

        Self { patterns }
    }

    /// Get the Go type for a field path
    pub fn get_go_type(&self, field_path: &str) -> Option<&String> {
        self.patterns.get(field_path)
    }

    /// Get Go type for a field with context
    pub fn get_contextual_type(&self, field_name: &str, context: &[&str]) -> Option<&String> {
        // Try full path first
        let full_path = format!("{}.{}", context.join("."), field_name);
        if let Some(go_type) = self.patterns.get(&full_path) {
            return Some(go_type);
        }

        // Try just the field name
        self.patterns.get(field_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_patterns() {
        let patterns = K8sTypePatterns::new();

        assert_eq!(
            patterns.get_go_type("metadata"),
            Some(&"k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta".to_string())
        );

        assert_eq!(
            patterns.get_contextual_type("volumes", &["spec"]),
            Some(&"[]k8s.io/api/core/v1.Volume".to_string())
        );
    }
}
