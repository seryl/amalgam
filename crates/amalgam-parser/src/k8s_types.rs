//! Kubernetes core types fetcher and generator

use crate::{imports::TypeReference, ParserError};
use amalgam_core::{
    ir::{Module, TypeDefinition},
    types::{Field, Type},
};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

/// Fetches and generates k8s.io core types
pub struct K8sTypesFetcher {
    client: reqwest::Client,
}

impl Default for K8sTypesFetcher {
    fn default() -> Self {
        Self::new()
    }
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

    /// Extract all k8s types from specific namespaces dynamically
    pub fn extract_core_types(
        &self,
        openapi: &Value,
    ) -> Result<HashMap<TypeReference, TypeDefinition>, ParserError> {
        let mut types = HashMap::new();

        // Namespaces we want to extract - these contain the most commonly used types
        let target_namespaces = [
            "io.k8s.apimachinery.pkg.apis.meta.v1",  // ObjectMeta, TypeMeta, etc.
            "io.k8s.api.core.v1",                     // Pod, Service, ConfigMap, etc.
            "io.k8s.api.apps.v1",                     // Deployment, StatefulSet, etc.
            "io.k8s.api.batch.v1",                    // Job, CronJob
            "io.k8s.api.networking.v1",               // Ingress, NetworkPolicy
            "io.k8s.api.rbac.v1",                     // Role, RoleBinding, etc.
            "io.k8s.api.storage.v1",                  // StorageClass
            "io.k8s.api.autoscaling.v1",              // HorizontalPodAutoscaler
            "io.k8s.api.policy.v1",                   // PodDisruptionBudget
            "io.k8s.apimachinery.pkg.api.resource",   // Quantity
        ];

        if let Some(definitions) = openapi.get("definitions").and_then(|d| d.as_object()) {
            // Iterate through all definitions
            for (full_name, schema) in definitions {
                // Check if this type is in one of our target namespaces
                let should_include = target_namespaces.iter().any(|&namespace| {
                    full_name.starts_with(namespace)
                });
                
                if should_include {
                    // Extract the short name from the full type name
                    let short_name = full_name
                        .split('.')
                        .next_back()
                        .unwrap_or(full_name.as_str())
                        .to_string();
                    
                    // Try to parse this as a k8s type reference
                    match self.parse_type_reference(full_name) {
                        Ok(type_ref) => {
                            match self.schema_to_type_definition(&short_name, schema) {
                                Ok(type_def) => {
                                    types.insert(type_ref, type_def);
                                }
                                Err(e) => {
                                    // Log but don't fail - some types might not parse correctly
                                    tracing::debug!("Failed to parse type {}: {}", full_name, e);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::debug!("Failed to parse reference {}: {}", full_name, e);
                        }
                    }
                }
            }
        }

        tracing::info!("Extracted {} k8s types from OpenAPI schema", types.len());
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

        let group = if parts[3] == "core" || parts[2] == "apimachinery" {
            "k8s.io".to_string() // core and apimachinery types are under k8s.io
        } else {
            format!("{}.k8s.io", parts[3])
        };

        let version = parts[parts.len() - 2].to_string();
        let kind = parts.last().unwrap().to_string();

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
            annotations: BTreeMap::new(),
        })
    }

    #[allow(clippy::only_used_in_recursion)]
    fn json_schema_to_type(&self, schema: &Value) -> Result<Type, ParserError> {
        // Check for top-level $ref first
        if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
            let type_name = ref_path.trim_start_matches("#/definitions/");
            
            // Resolve k8s type references to basic types
            return Ok(match type_name {
                name if name.ends_with(".Time") || name.ends_with(".MicroTime") => Type::String,
                name if name.ends_with(".Duration") => Type::String,
                name if name.ends_with(".IntOrString") => Type::Union(vec![Type::Integer, Type::String]),
                name if name.ends_with(".Quantity") => Type::String,
                name if name.ends_with(".FieldsV1") => Type::Any,
                name if name.starts_with("io.k8s.") => {
                    // For k8s internal references, use short name
                    let short_name = name.split('.').next_back().unwrap_or(name);
                    Type::Reference(short_name.to_string())
                }
                _ => Type::Reference(type_name.to_string())
            });
        }
        
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
                let mut fields = BTreeMap::new();

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
                            
                            // For k8s types, resolve common references to basic types
                            let resolved_type = match type_name {
                                // Time types should be strings
                                name if name.ends_with(".Time") || name.ends_with(".MicroTime") => {
                                    Type::String
                                }
                                // Duration is a string
                                name if name.ends_with(".Duration") => {
                                    Type::String
                                }
                                // IntOrString can be either
                                name if name.ends_with(".IntOrString") => {
                                    Type::Union(vec![Type::Integer, Type::String])
                                }
                                // Quantity is a string (like "100m" or "1Gi")
                                name if name.ends_with(".Quantity") || name == "io.k8s.apimachinery.pkg.api.resource.Quantity" => {
                                    Type::String
                                }
                                // FieldsV1 is a complex type, represent as Any for now
                                name if name.ends_with(".FieldsV1") => {
                                    Type::Any
                                }
                                // For other k8s references within the same module, keep as reference
                                // but only use the short name
                                name if name.starts_with("io.k8s.") => {
                                    // Extract just the type name (last part)
                                    let short_name = name.split('.').next_back().unwrap_or(name);
                                    Type::Reference(short_name.to_string())
                                }
                                // Keep full reference for non-k8s types
                                _ => Type::Reference(type_name.to_string())
                            };
                            
                            fields.insert(
                                field_name.clone(),
                                Field {
                                    ty: resolved_type,
                                    required: required.contains(field_name),
                                    description: field_schema
                                        .get("description")
                                        .and_then(|d| d.as_str())
                                        .map(String::from),
                                    default: None,
                                },
                            );
                        } else {
                            // Check if this is a type string reference
                            if field_schema.get("type").is_none() && field_schema.get("$ref").is_none() {
                                // Check for x-kubernetes fields or direct type strings
                                if let Value::String(type_str) = field_schema {
                                    // This is a direct type reference string
                                    let resolved_type = match type_str.as_str() {
                                        // Handle k8s type references
                                        s if s.ends_with(".Time") || s.ends_with(".MicroTime") => Type::String,
                                        s if s.ends_with(".Duration") => Type::String,
                                        s if s.ends_with(".IntOrString") => Type::Union(vec![Type::Integer, Type::String]),
                                        s if s.ends_with(".Quantity") => Type::String,
                                        s if s.ends_with(".FieldsV1") => Type::Any,
                                        s if s.starts_with("io.k8s.") => {
                                            // Extract just the type name (last part)
                                            let short_name = s.split('.').next_back().unwrap_or(s);
                                            Type::Reference(short_name.to_string())
                                        }
                                        _ => Type::Reference(type_str.clone())
                                    };
                                    
                                    fields.insert(
                                        field_name.clone(),
                                        Field {
                                            ty: resolved_type,
                                            required: required.contains(field_name),
                                            description: None,
                                            default: None,
                                        },
                                    );
                                    continue;
                                }
                            }
                            
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
                let mut fields = BTreeMap::new();
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
        annotations: BTreeMap::new(),
    };

    module.types.push(object_meta);

    // Add other common types...
    // This is simplified - in reality we'd fetch these from the k8s OpenAPI spec

    module
}
