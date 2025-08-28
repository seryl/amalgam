//! Kubernetes core types fetcher and generator

use crate::{imports::TypeReference, ParserError};
use amalgam_core::{
    ir::{Module, TypeDefinition},
    types::{Field, Type},
};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// Fetches and generates k8s.io core types
pub struct K8sTypesFetcher {
    client: reqwest::Client,
}

impl K8sTypesFetcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(60))
                .user_agent("amalgam")
                .build()
                .unwrap(),
        }
    }

    /// Fetch the Kubernetes OpenAPI schema
    pub async fn fetch_k8s_openapi(&self, version: &str) -> Result<Value, ParserError> {
        let is_tty = atty::is(atty::Stream::Stdout);

        let pb = if is_tty {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.cyan} {msg}")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_message(format!("Fetching Kubernetes {} OpenAPI schema...", version));
            Some(pb)
        } else {
            println!("Fetching Kubernetes {} OpenAPI schema...", version);
            None
        };

        // We can use the official k8s OpenAPI spec
        let url = format!(
            "https://raw.githubusercontent.com/kubernetes/kubernetes/{}/api/openapi-spec/swagger.json",
            version
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ParserError::Network(e.to_string()))?;

        if !response.status().is_success() {
            if let Some(pb) = pb {
                pb.finish_with_message(format!(
                    "✗ Failed to fetch k8s OpenAPI: {}",
                    response.status()
                ));
            }
            return Err(ParserError::Network(format!(
                "Failed to fetch k8s OpenAPI: {}",
                response.status()
            )));
        }

        if let Some(ref pb) = pb {
            pb.set_message("Parsing OpenAPI schema...");
        }

        let schema: Value = response
            .json()
            .await
            .map_err(|e| ParserError::Parse(e.to_string()))?;

        if let Some(pb) = pb {
            pb.finish_with_message(format!("✓ Fetched Kubernetes {} OpenAPI schema", version));
        } else {
            println!("Successfully fetched Kubernetes {} OpenAPI schema", version);
        }

        Ok(schema)
    }

    /// Extract common k8s types from OpenAPI schema
    pub fn extract_core_types(
        &self,
        openapi: &Value,
    ) -> Result<HashMap<TypeReference, TypeDefinition>, ParserError> {
        let mut types = HashMap::new();

        // Common types we want to extract
        let core_types = vec![
            (
                "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
                "ObjectMeta",
            ),
            ("io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta", "ListMeta"),
            ("io.k8s.apimachinery.pkg.apis.meta.v1.TypeMeta", "TypeMeta"),
            (
                "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector",
                "LabelSelector",
            ),
            ("io.k8s.api.core.v1.Volume", "Volume"),
            ("io.k8s.api.core.v1.VolumeMount", "VolumeMount"),
            ("io.k8s.api.core.v1.Container", "Container"),
            ("io.k8s.api.core.v1.PodSpec", "PodSpec"),
            (
                "io.k8s.api.core.v1.ResourceRequirements",
                "ResourceRequirements",
            ),
            ("io.k8s.api.core.v1.EnvVar", "EnvVar"),
            (
                "io.k8s.api.core.v1.ConfigMapKeySelector",
                "ConfigMapKeySelector",
            ),
            ("io.k8s.api.core.v1.SecretKeySelector", "SecretKeySelector"),
        ];

        if let Some(definitions) = openapi.get("definitions").and_then(|d| d.as_object()) {
            for (full_name, short_name) in core_types {
                if let Some(schema) = definitions.get(full_name) {
                    let type_ref = self.parse_type_reference(full_name)?;
                    let type_def = self.schema_to_type_definition(short_name, schema)?;
                    types.insert(type_ref, type_def);
                }
            }
        }

        Ok(types)
    }

    fn parse_type_reference(&self, full_name: &str) -> Result<TypeReference, ParserError> {
        // Parse "io.k8s.api.core.v1.Container" format
        let parts: Vec<&str> = full_name.split('.').collect();

        if parts.len() < 5 || parts[0] != "io" || parts[1] != "k8s" {
            return Err(ParserError::Parse(format!(
                "Invalid k8s type name: {}",
                full_name
            )));
        }

        let group = if parts[3] == "core" {
            "k8s.io".to_string()
        } else if parts[2] == "apimachinery" {
            "k8s.io".to_string() // apimachinery types are also under k8s.io
        } else {
            format!("{}.k8s.io", parts[3])
        };

        let version = parts[parts.len() - 2].to_string();
        let kind = parts[parts.len() - 1].to_string();

        Ok(TypeReference::new(group, version, kind))
    }

    fn schema_to_type_definition(
        &self,
        name: &str,
        schema: &Value,
    ) -> Result<TypeDefinition, ParserError> {
        let ty = self.json_schema_to_type(schema)?;

        Ok(TypeDefinition {
            name: name.to_string(),
            ty,
            documentation: schema
                .get("description")
                .and_then(|d| d.as_str())
                .map(String::from),
            annotations: HashMap::new(),
        })
    }

    fn json_schema_to_type(&self, schema: &Value) -> Result<Type, ParserError> {
        let schema_type = schema.get("type").and_then(|v| v.as_str());

        match schema_type {
            Some("string") => Ok(Type::String),
            Some("number") => Ok(Type::Number),
            Some("integer") => Ok(Type::Integer),
            Some("boolean") => Ok(Type::Bool),
            Some("array") => {
                let items = schema
                    .get("items")
                    .map(|i| self.json_schema_to_type(i))
                    .transpose()?
                    .unwrap_or(Type::Any);
                Ok(Type::Array(Box::new(items)))
            }
            Some("object") => {
                let mut fields = HashMap::new();

                if let Some(Value::Object(props)) = schema.get("properties") {
                    let required = schema
                        .get("required")
                        .and_then(|r| r.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(String::from)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    for (field_name, field_schema) in props {
                        // Check for $ref
                        if let Some(ref_path) = field_schema.get("$ref").and_then(|r| r.as_str()) {
                            // Convert ref to type reference
                            let type_name = ref_path.trim_start_matches("#/definitions/");
                            fields.insert(
                                field_name.clone(),
                                Field {
                                    ty: Type::Reference(type_name.to_string()),
                                    required: required.contains(field_name),
                                    description: field_schema
                                        .get("description")
                                        .and_then(|d| d.as_str())
                                        .map(String::from),
                                    default: None,
                                },
                            );
                        } else {
                            let field_type = self.json_schema_to_type(field_schema)?;
                            fields.insert(
                                field_name.clone(),
                                Field {
                                    ty: field_type,
                                    required: required.contains(field_name),
                                    description: field_schema
                                        .get("description")
                                        .and_then(|d| d.as_str())
                                        .map(String::from),
                                    default: field_schema.get("default").cloned(),
                                },
                            );
                        }
                    }
                }

                let open = schema
                    .get("additionalProperties")
                    .map(|v| !matches!(v, Value::Bool(false)))
                    .unwrap_or(false);

                Ok(Type::Record { fields, open })
            }
            _ => {
                // Check for $ref
                if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
                    let type_name = ref_path.trim_start_matches("#/definitions/");
                    Ok(Type::Reference(type_name.to_string()))
                } else {
                    Ok(Type::Any)
                }
            }
        }
    }
}

/// Generate a basic k8s.io package with common types
pub fn generate_k8s_package() -> Module {
    let mut module = Module {
        name: "k8s.io".to_string(),
        imports: Vec::new(),
        types: Vec::new(),
        constants: Vec::new(),
        metadata: Default::default(),
    };

    // Add ObjectMeta type (simplified)
    let object_meta = TypeDefinition {
        name: "ObjectMeta".to_string(),
        ty: Type::Record {
            fields: {
                let mut fields = HashMap::new();
                fields.insert(
                    "name".to_string(),
                    Field {
                        ty: Type::Optional(Box::new(Type::String)),
                        required: false,
                        description: Some("Name must be unique within a namespace".to_string()),
                        default: None,
                    },
                );
                fields.insert(
                    "namespace".to_string(),
                    Field {
                        ty: Type::Optional(Box::new(Type::String)),
                        required: false,
                        description: Some(
                            "Namespace defines the space within which each name must be unique"
                                .to_string(),
                        ),
                        default: None,
                    },
                );
                fields.insert(
                    "labels".to_string(),
                    Field {
                        ty: Type::Optional(Box::new(Type::Map {
                            key: Box::new(Type::String),
                            value: Box::new(Type::String),
                        })),
                        required: false,
                        description: Some(
                            "Map of string keys and values for organizing and categorizing objects"
                                .to_string(),
                        ),
                        default: None,
                    },
                );
                fields.insert(
                    "annotations".to_string(),
                    Field {
                        ty: Type::Optional(Box::new(Type::Map {
                            key: Box::new(Type::String),
                            value: Box::new(Type::String),
                        })),
                        required: false,
                        description: Some(
                            "Annotations is an unstructured key value map".to_string(),
                        ),
                        default: None,
                    },
                );
                fields.insert(
                    "uid".to_string(),
                    Field {
                        ty: Type::Optional(Box::new(Type::String)),
                        required: false,
                        description: Some(
                            "UID is the unique in time and space value for this object".to_string(),
                        ),
                        default: None,
                    },
                );
                fields.insert(
                    "resourceVersion".to_string(),
                    Field {
                        ty: Type::Optional(Box::new(Type::String)),
                        required: false,
                        description: Some(
                            "An opaque value that represents the internal version of this object"
                                .to_string(),
                        ),
                        default: None,
                    },
                );
                fields
            },
            open: true, // Allow additional fields
        },
        documentation: Some(
            "ObjectMeta is metadata that all persisted resources must have".to_string(),
        ),
        annotations: HashMap::new(),
    };

    module.types.push(object_meta);

    // Add other common types...
    // This is simplified - in reality we'd fetch these from the k8s OpenAPI spec

    module
}
