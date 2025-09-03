//! Swagger 2.0 parser for handling older API specifications

use crate::ParserError;
use amalgam_core::{
    ir::{IRBuilder, IR},
    types::{Field, Type},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Simplified Swagger 2.0 specification structure
#[derive(Debug, Deserialize, Serialize)]
pub struct SwaggerSpec {
    pub swagger: String,
    pub info: Option<Value>,
    pub paths: Option<Value>,
    pub definitions: Option<HashMap<String, Value>>,
}

pub struct SwaggerParser;

impl Default for SwaggerParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SwaggerParser {
    pub fn new() -> Self {
        Self
    }
}

/// Parse Swagger 2.0 JSON directly
pub fn parse_swagger_json(json_str: &str) -> Result<IR, ParserError> {
    // Parse as generic JSON and extract what we need
    let json: Value = serde_json::from_str(json_str)
        .map_err(|e| ParserError::InvalidInput(format!("Invalid JSON: {}", e)))?;

    parse_swagger_json_lenient(&json)
}

/// Lenient parser that extracts definitions from Swagger 2.0 JSON
fn parse_swagger_json_lenient(json: &Value) -> Result<IR, ParserError> {
    let mut builder = IRBuilder::new();

    // Check if it's Swagger 2.0
    if json.get("swagger").and_then(|v| v.as_str()) != Some("2.0") {
        return Err(ParserError::InvalidInput(
            "Not a Swagger 2.0 document".to_string(),
        ));
    }

    // Extract definitions and organize by module
    if let Some(definitions) = json.get("definitions").and_then(|d| d.as_object()) {
        // Group definitions by their module path
        let mut modules: std::collections::HashMap<String, Vec<(String, Type)>> =
            std::collections::HashMap::new();

        for (full_name, schema_json) in definitions {
            let ty = json_schema_to_type(schema_json)?;

            // Parse K8s-style names like "io.k8s.api.core.v1.Pod"
            // Extract module path and type name
            let (module_path, type_name) = parse_k8s_type_name(full_name);

            modules
                .entry(module_path.clone())
                .or_default()
                .push((type_name, ty));
        }

        // Add types to their respective modules
        for (module_path, types) in modules {
            builder = builder.module(&module_path);
            for (type_name, ty) in types {
                builder = builder.add_type(type_name, ty);
            }
        }
    }

    Ok(builder.build())
}

/// Parse a K8s-style type name into module path and type name
/// e.g., "io.k8s.api.core.v1.Pod" -> ("io.k8s.api.core.v1", "Pod")
fn parse_k8s_type_name(full_name: &str) -> (String, String) {
    // K8s types follow pattern: io.k8s.{api-group}.{version}.{Type}
    if full_name.starts_with("io.k8s.") {
        let parts: Vec<&str> = full_name.split('.').collect();

        if parts.len() > 1 {
            // Last part is the type name
            let type_name = parts.last().unwrap().to_string();
            // Everything before the last part is the module path
            // Keep the full io.k8s.* prefix for proper normalization later
            let module_parts = &parts[..parts.len() - 1];
            let module_path = module_parts.join(".");
            return (module_path, type_name);
        }
    }

    // For non-K8s types or types without proper namespacing,
    // use a default module
    ("types".to_string(), full_name.to_string())
}

/// Convert JSON schema to Type
fn json_schema_to_type(schema: &Value) -> Result<Type, ParserError> {
    // Handle $ref
    if let Some(ref_str) = schema.get("$ref").and_then(|r| r.as_str()) {
        if let Some(type_name) = ref_str.strip_prefix("#/definitions/") {
            return Ok(Type::Reference {
                name: type_name.to_string(),
                module: None,
            });
        }
    }

    // Handle type field
    match schema.get("type").and_then(|t| t.as_str()) {
        Some("string") => Ok(Type::String),
        Some("number") => Ok(Type::Number),
        Some("integer") => Ok(Type::Integer),
        Some("boolean") => Ok(Type::Bool),
        Some("array") => {
            let item_type = schema
                .get("items")
                .map(json_schema_to_type)
                .transpose()?
                .unwrap_or(Type::Any);
            Ok(Type::Array(Box::new(item_type)))
        }
        Some("object") => {
            let mut fields = BTreeMap::new();

            if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
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

                for (field_name, field_schema) in properties {
                    let field_type = json_schema_to_type(field_schema)?;

                    fields.insert(
                        field_name.clone(),
                        Field {
                            ty: field_type,
                            required: required.contains(field_name),
                            description: field_schema
                                .get("description")
                                .and_then(|d| d.as_str())
                                .map(String::from),
                            default: field_schema
                                .get("default")
                                .and_then(|v| serde_json::from_value(v.clone()).ok()),
                        },
                    );
                }
            }

            Ok(Type::Record {
                fields,
                open: schema.get("additionalProperties").is_some(),
            })
        }
        _ => {
            // Check for composition keywords
            if schema.get("allOf").is_some() {
                Ok(Type::Any)
            } else if let Some(one_of) = schema.get("oneOf").and_then(|o| o.as_array()) {
                let mut types = Vec::new();
                for schema_ref in one_of {
                    types.push(json_schema_to_type(schema_ref)?);
                }
                Ok(Type::Union {
                    types,
                    coercion_hint: None,
                })
            } else if let Some(any_of) = schema.get("anyOf").and_then(|a| a.as_array()) {
                let mut types = Vec::new();
                for schema_ref in any_of {
                    types.push(json_schema_to_type(schema_ref)?);
                }
                Ok(Type::Union {
                    types,
                    coercion_hint: None,
                })
            } else {
                Ok(Type::Any)
            }
        }
    }
}
