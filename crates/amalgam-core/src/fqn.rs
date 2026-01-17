//! Unified Fully-Qualified Name (FQN) parser for consistent type reference handling
//!
//! This module provides a single source of truth for parsing fully-qualified type names
//! across the codebase, replacing scattered parsing logic in:
//! - `imports.rs`
//! - `package_walker.rs`
//! - `nickel.rs`
//! - `module_registry.rs`
//!
//! ## FQN Formats Supported
//!
//! - Standard: `io.k8s.api.core.v1.Pod`
//! - K8s Short: `k8s.io.v1.ObjectMeta`
//! - CrossPlane: `apiextensions.crossplane.io.v1.Composition`
//! - Legacy: `io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta`
//!
//! ## Usage
//!
//! ```ignore
//! let fqn = Fqn::parse("io.k8s.api.core.v1.Pod")?;
//! assert_eq!(fqn.type_name(), "Pod");
//! assert_eq!(fqn.module(), "io.k8s.api.core.v1");
//! assert_eq!(fqn.version(), "v1");
//! assert_eq!(fqn.group(), "io.k8s.api.core");
//! ```

use std::fmt;

use serde::{Deserialize, Serialize};

/// A parsed fully-qualified name
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fqn {
    /// The full original FQN string
    original: String,
    /// The type name (e.g., "Pod", "ObjectMeta")
    type_name: String,
    /// The module path (e.g., "io.k8s.api.core.v1")
    module: String,
    /// The group/package (e.g., "io.k8s.api.core", "k8s.io")
    group: String,
    /// The version (e.g., "v1", "v1alpha3")
    version: String,
    /// The domain (e.g., "k8s.io", "crossplane.io")
    domain: String,
    /// The namespace within the domain (e.g., "api.core", "apiextensions")
    namespace: String,
}

/// Errors that can occur during FQN parsing
#[derive(Debug, Clone, thiserror::Error)]
pub enum FqnError {
    #[error("Empty FQN string")]
    Empty,

    #[error("Invalid FQN format: {0}")]
    InvalidFormat(String),

    #[error("Missing type name in FQN: {0}")]
    MissingTypeName(String),

    #[error("Could not determine version in FQN: {0}")]
    MissingVersion(String),
}

impl Fqn {
    /// Parse a fully-qualified name string
    ///
    /// Supports multiple formats:
    /// - `io.k8s.api.core.v1.Pod`
    /// - `k8s.io.v1.ObjectMeta`
    /// - `apiextensions.crossplane.io.v1.Composition`
    pub fn parse(fqn: &str) -> Result<Self, FqnError> {
        if fqn.is_empty() {
            return Err(FqnError::Empty);
        }

        let parts: Vec<&str> = fqn.split('.').collect();

        // Handle single-word type names (e.g., "Pod")
        if parts.len() == 1 {
            let type_name = parts[0].to_string();
            return Ok(Self {
                original: fqn.to_string(),
                type_name: type_name.clone(),
                module: String::new(),
                group: String::new(),
                version: "v1".to_string(), // Default version
                domain: "local://".to_string(),
                namespace: "default".to_string(),
            });
        }

        // Find the type name (last part that starts with uppercase)
        let type_name_idx = parts.iter().rposition(|p| {
            p.chars().next().map_or(false, |c| c.is_ascii_uppercase())
        });

        let type_name = match type_name_idx {
            Some(idx) => parts[idx].to_string(),
            None => {
                // If no uppercase part, treat the last part as the type name
                parts.last().unwrap().to_string()
            }
        };

        // Find the version (a part starting with 'v' followed by a digit, or special versions)
        let version_idx = parts.iter().rposition(|p| Self::is_version(p));

        let (module, group, version) = match version_idx {
            Some(v_idx) => {
                let type_idx = type_name_idx.unwrap_or(parts.len());
                if v_idx >= type_idx {
                    // Version is after the type name - treat everything before type as module
                    let module = parts[..type_idx].join(".");
                    let version = if v_idx < type_idx {
                        parts[v_idx].to_string()
                    } else {
                        "v1".to_string()
                    };
                    let group = if v_idx > 0 {
                        parts[..v_idx].join(".")
                    } else {
                        module.clone()
                    };
                    (module, group, version)
                } else {
                    // Normal case: module includes version
                    let module_end = type_name_idx.unwrap_or(parts.len());
                    let module = parts[..module_end].join(".");
                    let version = parts[v_idx].to_string();
                    let group = parts[..v_idx].join(".");
                    (module, group, version)
                }
            }
            None => {
                // No version found - assume it's all group
                let type_idx = type_name_idx.unwrap_or(parts.len());
                let module = parts[..type_idx].join(".");
                (module.clone(), module, "v1".to_string())
            }
        };

        // Extract domain and namespace
        let (domain, namespace) = Self::extract_domain_namespace(&group);

        Ok(Self {
            original: fqn.to_string(),
            type_name,
            module,
            group,
            version,
            domain,
            namespace,
        })
    }

    /// Check if a string is a version identifier
    fn is_version(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        // Standard versions: v1, v1alpha1, v1beta1, v0
        if s.starts_with('v') && s.len() > 1 {
            let rest = &s[1..];
            return rest.chars().next().map_or(false, |c| c.is_ascii_digit());
        }

        // Special "versions" that act as version-like path components
        matches!(s, "v0" | "resource" | "crossplane")
    }

    /// Extract domain and namespace from a group string
    fn extract_domain_namespace(group: &str) -> (String, String) {
        if group.is_empty() {
            return ("local://".to_string(), "core".to_string());
        }

        let parts: Vec<&str> = group.split('.').collect();

        // Handle io.k8s.* format (legacy K8s)
        if group.starts_with("io.k8s.") {
            // Extract namespace after io.k8s.
            let namespace = group.strip_prefix("io.k8s.").unwrap_or("").to_string();
            return ("k8s.io".to_string(), namespace);
        }

        // Handle k8s.io format
        if group == "k8s.io" {
            return ("k8s.io".to_string(), "core".to_string());
        }

        if group.starts_with("k8s.io.") {
            let namespace = group.strip_prefix("k8s.io.").unwrap_or("").to_string();
            return ("k8s.io".to_string(), namespace);
        }

        // Check for well-known TLDs
        if parts.len() >= 2 {
            let tld = parts[parts.len() - 1];
            if matches!(tld, "io" | "com" | "org" | "net" | "dev" | "app") {
                // Domain is the last 2 parts (or 3 for known projects)
                let domain_parts = if parts.len() >= 3
                    && matches!(
                        parts[parts.len() - 2],
                        "crossplane" | "kubernetes" | "istio" | "linkerd"
                    )
                {
                    2
                } else {
                    2
                };

                let domain = parts[parts.len() - domain_parts..].join(".");
                let namespace = if parts.len() > domain_parts {
                    parts[0..parts.len() - domain_parts].join(".")
                } else {
                    "default".to_string()
                };

                return (domain, namespace);
            }
        }

        // Fallback: treat as local package
        (format!("local://{}", group), "default".to_string())
    }

    /// Get the original FQN string
    pub fn original(&self) -> &str {
        &self.original
    }

    /// Get the type name (e.g., "Pod")
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Get the module path (e.g., "io.k8s.api.core.v1")
    pub fn module(&self) -> &str {
        &self.module
    }

    /// Get the group/package (e.g., "io.k8s.api.core")
    pub fn group(&self) -> &str {
        &self.group
    }

    /// Get the version (e.g., "v1")
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the domain (e.g., "k8s.io")
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Get the namespace (e.g., "api.core")
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Check if this is a K8s type
    pub fn is_k8s(&self) -> bool {
        self.domain == "k8s.io"
    }

    /// Check if this is a Crossplane type
    pub fn is_crossplane(&self) -> bool {
        self.domain == "crossplane.io"
    }

    /// Get the API group for K8s types (e.g., "apps", "core", "batch")
    pub fn k8s_api_group(&self) -> Option<&str> {
        if !self.is_k8s() {
            return None;
        }

        // For io.k8s.api.{group} format
        if self.namespace.starts_with("api.") {
            return self.namespace.strip_prefix("api.").and_then(|s| s.split('.').next());
        }

        // For k8s.io.{group} format
        if !self.namespace.is_empty() && self.namespace != "core" && self.namespace != "default" {
            return Some(self.namespace.split('.').next().unwrap_or(&self.namespace));
        }

        Some("core")
    }

    /// Create a new FQN with a different type name
    pub fn with_type_name(&self, type_name: &str) -> Self {
        let new_original = format!("{}.{}", self.module, type_name);
        Self {
            original: new_original,
            type_name: type_name.to_string(),
            module: self.module.clone(),
            group: self.group.clone(),
            version: self.version.clone(),
            domain: self.domain.clone(),
            namespace: self.namespace.clone(),
        }
    }

    /// Normalize the FQN to a canonical format
    ///
    /// Converts various formats to a standard form:
    /// - `io.k8s.api.core.v1.Pod` -> `k8s.io.core.v1.Pod`
    pub fn normalize(&self) -> Self {
        // For K8s types, normalize to k8s.io.{group}.{version}.{type} format
        if self.is_k8s() && self.original.starts_with("io.k8s.") {
            let api_group = self.k8s_api_group().unwrap_or("core");
            let normalized = format!(
                "k8s.io.{}.{}.{}",
                api_group, self.version, self.type_name
            );
            // Re-parse the normalized FQN
            Self::parse(&normalized).unwrap_or_else(|_| self.clone())
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for Fqn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}

impl From<&str> for Fqn {
    fn from(s: &str) -> Self {
        Self::parse(s).unwrap_or_else(|_| Self {
            original: s.to_string(),
            type_name: s.to_string(),
            module: String::new(),
            group: String::new(),
            version: "v1".to_string(),
            domain: "unknown".to_string(),
            namespace: "default".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_k8s_legacy_format() {
        let fqn = Fqn::parse("io.k8s.api.core.v1.Pod").unwrap();
        assert_eq!(fqn.type_name(), "Pod");
        assert_eq!(fqn.module(), "io.k8s.api.core.v1");
        assert_eq!(fqn.group(), "io.k8s.api.core");
        assert_eq!(fqn.version(), "v1");
        assert_eq!(fqn.domain(), "k8s.io");
        assert_eq!(fqn.namespace(), "api.core");
        assert!(fqn.is_k8s());
        assert_eq!(fqn.k8s_api_group(), Some("core"));
    }

    #[test]
    fn test_parse_k8s_short_format() {
        let fqn = Fqn::parse("k8s.io.v1.ObjectMeta").unwrap();
        assert_eq!(fqn.type_name(), "ObjectMeta");
        assert_eq!(fqn.version(), "v1");
        assert_eq!(fqn.domain(), "k8s.io");
        assert!(fqn.is_k8s());
    }

    #[test]
    fn test_parse_k8s_apps() {
        let fqn = Fqn::parse("io.k8s.api.apps.v1.Deployment").unwrap();
        assert_eq!(fqn.type_name(), "Deployment");
        assert_eq!(fqn.version(), "v1");
        assert_eq!(fqn.k8s_api_group(), Some("apps"));
    }

    #[test]
    fn test_parse_k8s_apimachinery() {
        let fqn = Fqn::parse("io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta").unwrap();
        assert_eq!(fqn.type_name(), "ObjectMeta");
        assert_eq!(fqn.version(), "v1");
        assert_eq!(fqn.domain(), "k8s.io");
    }

    #[test]
    fn test_parse_crossplane() {
        let fqn = Fqn::parse("apiextensions.crossplane.io.v1.Composition").unwrap();
        assert_eq!(fqn.type_name(), "Composition");
        assert_eq!(fqn.version(), "v1");
        assert_eq!(fqn.domain(), "crossplane.io");
        assert_eq!(fqn.namespace(), "apiextensions");
        assert!(fqn.is_crossplane());
    }

    #[test]
    fn test_parse_alpha_version() {
        let fqn = Fqn::parse("io.k8s.api.batch.v1beta1.CronJob").unwrap();
        assert_eq!(fqn.type_name(), "CronJob");
        assert_eq!(fqn.version(), "v1beta1");
    }

    #[test]
    fn test_parse_empty() {
        assert!(matches!(Fqn::parse(""), Err(FqnError::Empty)));
    }

    #[test]
    fn test_parse_single_word() {
        let fqn = Fqn::parse("Pod").unwrap();
        assert_eq!(fqn.type_name(), "Pod");
        assert_eq!(fqn.version(), "v1"); // Default version
    }

    #[test]
    fn test_is_version() {
        assert!(Fqn::is_version("v1"));
        assert!(Fqn::is_version("v1alpha1"));
        assert!(Fqn::is_version("v1beta1"));
        assert!(Fqn::is_version("v0"));
        assert!(Fqn::is_version("v2"));
        assert!(!Fqn::is_version(""));
        assert!(!Fqn::is_version("version"));
        assert!(!Fqn::is_version("Pod"));
    }

    #[test]
    fn test_normalize() {
        let fqn = Fqn::parse("io.k8s.api.core.v1.Pod").unwrap();
        let normalized = fqn.normalize();
        // Should normalize to k8s.io format
        assert!(normalized.original().starts_with("k8s.io"));
        assert_eq!(normalized.type_name(), "Pod");
    }

    #[test]
    fn test_with_type_name() {
        let fqn = Fqn::parse("io.k8s.api.core.v1.Pod").unwrap();
        let new_fqn = fqn.with_type_name("Service");
        assert_eq!(new_fqn.type_name(), "Service");
        assert_eq!(new_fqn.module(), fqn.module());
    }

    #[test]
    fn test_display() {
        let fqn = Fqn::parse("io.k8s.api.core.v1.Pod").unwrap();
        assert_eq!(format!("{}", fqn), "io.k8s.api.core.v1.Pod");
    }
}
