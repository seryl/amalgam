//! Import resolution for cross-package type references

use amalgam_core::ImportPathCalculator;

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
        Self {
            group,
            version,
            kind,
        }
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
        } else if name.starts_with("v1.")
            || name.starts_with("v1beta1.")
            || name.starts_with("v1alpha1.")
        {
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
    pub fn import_path(&self, from_group: &str, from_version: &str) -> String {
        let calc = ImportPathCalculator::new_standalone();
        calc.calculate(
            from_group,
            from_version,
            &self.group,
            &self.version,
            &self.kind,
        )
    }

    /// Get the module alias for imports
    pub fn module_alias(&self) -> String {
        format!(
            "{}_{}",
            self.group.replace(['.', '-'], "_"),
            self.version.replace('-', "_")
        )
    }
}

/// Common Kubernetes types that are frequently referenced
pub fn common_k8s_types() -> Vec<TypeReference> {
    vec![
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "ObjectMeta".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "ListMeta".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "TypeMeta".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "LabelSelector".to_string(),
        ),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "Volume".to_string()),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "VolumeMount".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "Container".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "PodSpec".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "ResourceRequirements".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "Affinity".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "Toleration".to_string(),
        ),
        TypeReference::new("k8s.io".to_string(), "v1".to_string(), "EnvVar".to_string()),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "ConfigMapKeySelector".to_string(),
        ),
        TypeReference::new(
            "k8s.io".to_string(),
            "v1".to_string(),
            "SecretKeySelector".to_string(),
        ),
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

        // With our unified ImportPathCalculator, CrossPlane packages have nested structure
        // crossplane/apiextensions.crossplane.io/crossplane/<version>/file.ncl
        // So we go up 4 levels to reach the packages root
        let path = type_ref.import_path("apiextensions.crossplane.io", "v1");
        assert_eq!(path, "../../../../k8s_io/v1/objectmeta.ncl");

        // Test with a simple group - same path structure
        let path2 = type_ref.import_path("example.io", "v1");
        assert_eq!(path2, "../../k8s_io/v1/objectmeta.ncl");

        // Test same-package cross-version
        let path3 = type_ref.import_path("k8s.io", "v1beta1");
        assert_eq!(path3, "../v1/objectmeta.ncl");

        // Test same-package same-version
        let path4 = type_ref.import_path("k8s.io", "v1");
        assert_eq!(path4, "./objectmeta.ncl");
    }
}
