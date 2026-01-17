//! OpenAPI/JSON Schema parser

use crate::{Parser, ParserError};
use amalgam_core::{
    ir::{IRBuilder, IR},
    types::{Field, Type, ValidationRules},
};
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type as OpenAPIType};
use std::collections::BTreeMap;

pub struct OpenAPIParser;

impl Parser for OpenAPIParser {
    type Input = OpenAPI;

    fn parse(&self, input: Self::Input) -> Result<IR, ParserError> {
        let mut builder = IRBuilder::new().module("openapi");

        // Parse components/schemas
        if let Some(components) = input.components {
            for (name, schema_ref) in components.schemas {
                if let openapiv3::ReferenceOr::Item(schema) = schema_ref {
                    let ty = self.schema_to_type(&schema)?;
                    builder = builder.add_type(name, ty);
                }
            }
        }

        Ok(builder.build())
    }
}

impl OpenAPIParser {
    pub fn new() -> Self {
        Self
    }

    /// Extract validation rules from an openapiv3 Schema
    fn extract_validation_rules(&self, schema: &Schema) -> Option<ValidationRules> {
        let mut rules = ValidationRules::default();
        let mut has_rules = false;

        match &schema.schema_kind {
            SchemaKind::Type(OpenAPIType::String(string_type)) => {
                if let Some(min_len) = string_type.min_length {
                    rules.min_length = Some(min_len);
                    has_rules = true;
                }
                if let Some(max_len) = string_type.max_length {
                    rules.max_length = Some(max_len);
                    has_rules = true;
                }
                if let Some(ref pattern) = string_type.pattern {
                    rules.pattern = Some(pattern.clone());
                    has_rules = true;
                }
                if !string_type.enumeration.is_empty() {
                    let values: Vec<serde_json::Value> = string_type
                        .enumeration
                        .iter()
                        .filter_map(|v| v.as_ref())
                        .map(|s| serde_json::Value::String(s.clone()))
                        .collect();
                    if !values.is_empty() {
                        rules.allowed_values = Some(values);
                        has_rules = true;
                    }
                }
            }
            SchemaKind::Type(OpenAPIType::Integer(int_type)) => {
                if let Some(min) = int_type.minimum {
                    rules.minimum = Some(min as f64);
                    has_rules = true;
                }
                if let Some(max) = int_type.maximum {
                    rules.maximum = Some(max as f64);
                    has_rules = true;
                }
                if int_type.exclusive_minimum {
                    rules.exclusive_minimum = rules.minimum;
                    rules.minimum = None;
                }
                if int_type.exclusive_maximum {
                    rules.exclusive_maximum = rules.maximum;
                    rules.maximum = None;
                }
                if !int_type.enumeration.is_empty() {
                    let values: Vec<serde_json::Value> = int_type
                        .enumeration
                        .iter()
                        .filter_map(|v| v.map(|n| serde_json::Value::Number(n.into())))
                        .collect();
                    if !values.is_empty() {
                        rules.allowed_values = Some(values);
                        has_rules = true;
                    }
                }
            }
            SchemaKind::Type(OpenAPIType::Number(num_type)) => {
                if let Some(min) = num_type.minimum {
                    rules.minimum = Some(min);
                    has_rules = true;
                }
                if let Some(max) = num_type.maximum {
                    rules.maximum = Some(max);
                    has_rules = true;
                }
                if num_type.exclusive_minimum {
                    rules.exclusive_minimum = rules.minimum;
                    rules.minimum = None;
                }
                if num_type.exclusive_maximum {
                    rules.exclusive_maximum = rules.maximum;
                    rules.maximum = None;
                }
            }
            SchemaKind::Type(OpenAPIType::Array(array_type)) => {
                if let Some(min) = array_type.min_items {
                    rules.min_items = Some(min);
                    has_rules = true;
                }
                if let Some(max) = array_type.max_items {
                    rules.max_items = Some(max);
                    has_rules = true;
                }
                if array_type.unique_items {
                    rules.unique_items = Some(true);
                    has_rules = true;
                }
            }
            _ => {}
        }

        if has_rules {
            Some(rules)
        } else {
            None
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn schema_to_type(&self, schema: &Schema) -> Result<Type, ParserError> {
        match &schema.schema_kind {
            SchemaKind::Type(OpenAPIType::String(_)) => Ok(Type::String),
            SchemaKind::Type(OpenAPIType::Number(_)) => Ok(Type::Number),
            SchemaKind::Type(OpenAPIType::Integer(_)) => Ok(Type::Integer),
            SchemaKind::Type(OpenAPIType::Boolean(_)) => Ok(Type::Bool),
            SchemaKind::Type(OpenAPIType::Array(array_type)) => {
                let item_type = if let Some(ReferenceOr::Item(item_schema)) = &array_type.items {
                    self.schema_to_type(item_schema)?
                } else {
                    Type::Any
                };
                Ok(Type::Array(Box::new(item_type)))
            }
            SchemaKind::Type(OpenAPIType::Object(object_type)) => {
                let mut fields = BTreeMap::new();
                for (field_name, field_schema_ref) in &object_type.properties {
                    if let ReferenceOr::Item(field_schema) = field_schema_ref {
                        let field_type = self.schema_to_type(field_schema)?;
                        let required = object_type.required.contains(field_name);
                        let validation = self.extract_validation_rules(field_schema);
                        fields.insert(
                            field_name.clone(),
                            Field {
                                ty: field_type,
                                required,
                                description: field_schema.schema_data.description.clone(),
                                default: None,
                                validation,
                                contracts: Vec::new(),
                            },
                        );
                    }
                }
                // Check if this is a map type (object with additionalProperties defining value type)
                // If there are no explicit properties and additionalProperties defines a schema,
                // this is a map type (like ConfigMap data: map[string]string)
                if fields.is_empty() {
                    if let Some(additional_props) = &object_type.additional_properties {
                        match additional_props {
                            openapiv3::AdditionalProperties::Schema(schema_ref) => {
                                if let ReferenceOr::Item(schema) = schema_ref.as_ref() {
                                    let value_type = self.schema_to_type(schema)?;
                                    return Ok(Type::Map {
                                        key: Box::new(Type::String),
                                        value: Box::new(value_type),
                                    });
                                }
                            }
                            openapiv3::AdditionalProperties::Any(true) => {
                                return Ok(Type::Map {
                                    key: Box::new(Type::String),
                                    value: Box::new(Type::Any),
                                });
                            }
                            _ => {}
                        }
                    }
                }

                Ok(Type::Record {
                    fields,
                    open: object_type.additional_properties.is_some(),
                })
            }
            SchemaKind::OneOf { one_of } => {
                let mut types = Vec::new();
                for schema_ref in one_of {
                    if let ReferenceOr::Item(schema) = schema_ref {
                        types.push(self.schema_to_type(schema)?);
                    }
                }
                Ok(Type::Union {
                    types,
                    coercion_hint: None,
                })
            }
            SchemaKind::AllOf { all_of: _ } => {
                // For now, treat as Any - would need more complex merging
                Ok(Type::Any)
            }
            SchemaKind::AnyOf { any_of } => {
                let mut types = Vec::new();
                for schema_ref in any_of {
                    if let ReferenceOr::Item(schema) = schema_ref {
                        types.push(self.schema_to_type(schema)?);
                    }
                }
                Ok(Type::Union {
                    types,
                    coercion_hint: None,
                })
            }
            SchemaKind::Not { .. } => {
                Err(ParserError::UnsupportedFeature("'not' schema".to_string()))
            }
            SchemaKind::Any(_) => Ok(Type::Any),
        }
    }
}

impl Default for OpenAPIParser {
    fn default() -> Self {
        Self::new()
    }
}
