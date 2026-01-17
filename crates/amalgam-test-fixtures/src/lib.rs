//! Test fixtures for Amalgam compiler testing
//!
//! Provides minimal, representative test data without storing full outputs

use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

/// Test fixture categories
pub enum FixtureType {
    MinimalK8s,
    CompleteK8s,
    SimpleCrd,
    ComplexCrd,
    CrossplaneCrd,
    EdgeCases,
}

/// Main test fixtures provider
pub struct TestFixtures {
    temp_dir: Option<tempfile::TempDir>,
}

impl Default for TestFixtures {
    fn default() -> Self {
        Self::new()
    }
}

impl TestFixtures {
    pub fn new() -> Self {
        Self { temp_dir: None }
    }

    /// Create a temporary directory with test fixtures
    pub fn setup(&mut self, fixture_type: FixtureType) -> PathBuf {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_path_buf();

        match fixture_type {
            FixtureType::MinimalK8s => self.setup_minimal_k8s(&path),
            FixtureType::CompleteK8s => self.setup_complete_k8s(&path),
            FixtureType::SimpleCrd => self.setup_simple_crd(&path),
            FixtureType::ComplexCrd => self.setup_complex_crd(&path),
            FixtureType::CrossplaneCrd => self.setup_crossplane_crd(&path),
            FixtureType::EdgeCases => self.setup_edge_cases(&path),
        }

        self.temp_dir = Some(dir);
        path
    }

    fn setup_minimal_k8s(&self, base: &Path) {
        // Create minimal K8s structure
        let k8s_dir = base.join("k8s_io");
        fs::create_dir_all(&k8s_dir).unwrap();

        // Write minimal types
        fs::write(
            k8s_dir.join("v1.ncl"),
            r#"# Minimal K8s types
{
  Pod = {
    apiVersion | String,
    kind | String,
    metadata | optional,
    spec = {
      containers | Array {
        name | String,
        image | String,
      },
    },
  },
  Service = {
    apiVersion | String,
    kind | String,
    metadata | optional,
    spec = {
      selector | optional,
      ports | optional | Array { port | Number },
    },
  },
}"#,
        )
        .unwrap();

        fs::write(
            k8s_dir.join("mod.ncl"),
            r#"{
  v1 = import "./v1.ncl",
}"#,
        )
        .unwrap();
    }

    fn setup_complete_k8s(&self, _base: &Path) {
        // Would set up more complete K8s types for integration testing
    }

    fn setup_simple_crd(&self, base: &Path) {
        let crd_dir = base.join("crds");
        fs::create_dir_all(&crd_dir).unwrap();

        fs::write(
            crd_dir.join("simple.yaml"),
            r#"apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: samples.example.io
spec:
  group: example.io
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              replicas:
                type: integer
                minimum: 1
                maximum: 10
              message:
                type: string
                maxLength: 100
"#,
        )
        .unwrap();
    }

    fn setup_complex_crd(&self, _base: &Path) {
        // Complex CRD with nested types, refs, etc.
    }

    fn setup_crossplane_crd(&self, base: &Path) {
        let crd_dir = base.join("crds");
        fs::create_dir_all(&crd_dir).unwrap();

        // Minimal Crossplane Composition CRD
        fs::write(
            crd_dir.join("composition.yaml"),
            r#"apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: compositions.apiextensions.crossplane.io
spec:
  group: apiextensions.crossplane.io
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              compositeTypeRef:
                type: object
                properties:
                  apiVersion:
                    type: string
                  kind:
                    type: string
              resources:
                type: array
                items:
                  type: object
                  properties:
                    name:
                      type: string
                    base:
                      type: object
                      x-kubernetes-preserve-unknown-fields: true
"#,
        )
        .unwrap();
    }

    fn setup_edge_cases(&self, base: &Path) {
        let edge_dir = base.join("edge_cases");
        fs::create_dir_all(&edge_dir).unwrap();

        // Type with intOrString
        fs::write(
            edge_dir.join("intorstring.yaml"),
            r#"apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: ports.example.io
spec:
  group: example.io
  versions:
  - name: v1
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              port:
                x-kubernetes-int-or-string: true
"#,
        )
        .unwrap();
    }
}

/// Minimal Kubernetes swagger for testing core functionality
pub fn minimal_k8s_swagger() -> serde_json::Value {
    json!({
        "swagger": "2.0",
        "info": {
            "title": "Kubernetes API",
            "version": "v1.33.4"
        },
        "paths": {},
        "definitions": {
            "io.k8s.api.core.v1.Pod": {
                "type": "object",
                "properties": {
                    "apiVersion": {"type": "string"},
                    "kind": {"type": "string"},
                    "metadata": {"$ref": "#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"},
                    "spec": {"$ref": "#/definitions/io.k8s.api.core.v1.PodSpec"}
                }
            },
            "io.k8s.api.core.v1.PodSpec": {
                "type": "object",
                "properties": {
                    "containers": {
                        "type": "array",
                        "items": {"$ref": "#/definitions/io.k8s.api.core.v1.Container"}
                    }
                }
            },
            "io.k8s.api.core.v1.Container": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "image": {"type": "string"},
                    "ports": {
                        "type": "array",
                        "items": {"$ref": "#/definitions/io.k8s.api.core.v1.ContainerPort"}
                    }
                }
            },
            "io.k8s.api.core.v1.ContainerPort": {
                "type": "object",
                "properties": {
                    "containerPort": {"type": "integer", "format": "int32"},
                    "protocol": {"type": "string"}
                }
            },
            "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "namespace": {"type": "string"},
                    "labels": {
                        "type": "object",
                        "additionalProperties": {"type": "string"}
                    }
                }
            }
        }
    })
}

/// Minimal CRD for testing
pub fn minimal_crd() -> serde_yaml::Value {
    serde_yaml::from_str(
        r#"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: samples.example.io
spec:
  group: example.io
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              replicas:
                type: integer
              message:
                type: string
"#,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixtures_are_valid() {
        let swagger = minimal_k8s_swagger();
        assert!(swagger["definitions"].is_object());

        let crd = minimal_crd();
        assert_eq!(crd["kind"], "CustomResourceDefinition");
    }
}
