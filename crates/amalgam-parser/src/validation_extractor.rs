//! Validation rule extraction from JSON Schema
//!
//! This module provides utilities to extract validation rules from JSON Schema
//! definitions and convert them to Amalgam's ValidationRules structure.

use amalgam_core::types::{ContractRule, StringFormat, ValidationRules};
use serde_json::Value;

/// Extracts validation rules from a JSON Schema object
pub struct ValidationExtractor;

impl ValidationExtractor {
    /// Extract validation rules from a JSON Schema value
    pub fn extract_validation_rules(schema: &Value) -> ValidationRules {
        let mut rules = ValidationRules::default();

        // Extract string validation rules
        if let Some(min_length) = schema.get("minLength").and_then(|v| v.as_u64()) {
            rules.min_length = Some(min_length as usize);
        }

        if let Some(max_length) = schema.get("maxLength").and_then(|v| v.as_u64()) {
            rules.max_length = Some(max_length as usize);
        }

        if let Some(pattern) = schema.get("pattern").and_then(|v| v.as_str()) {
            rules.pattern = Some(pattern.to_string());
        }

        if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
            rules.format = Self::parse_string_format(format);
        }

        // Extract number validation rules
        if let Some(minimum) = schema.get("minimum").and_then(|v| v.as_f64()) {
            rules.minimum = Some(minimum);
        }

        if let Some(maximum) = schema.get("maximum").and_then(|v| v.as_f64()) {
            rules.maximum = Some(maximum);
        }

        if let Some(exclusive_min) = schema.get("exclusiveMinimum").and_then(|v| v.as_f64()) {
            rules.exclusive_minimum = Some(exclusive_min);
        }

        if let Some(exclusive_max) = schema.get("exclusiveMaximum").and_then(|v| v.as_f64()) {
            rules.exclusive_maximum = Some(exclusive_max);
        }

        // Extract array validation rules
        if let Some(min_items) = schema.get("minItems").and_then(|v| v.as_u64()) {
            rules.min_items = Some(min_items as usize);
        }

        if let Some(max_items) = schema.get("maxItems").and_then(|v| v.as_u64()) {
            rules.max_items = Some(max_items as usize);
        }

        if let Some(unique_items) = schema.get("uniqueItems").and_then(|v| v.as_bool()) {
            rules.unique_items = Some(unique_items);
        }

        // Extract object validation rules
        if let Some(min_props) = schema.get("minProperties").and_then(|v| v.as_u64()) {
            rules.min_properties = Some(min_props as usize);
        }

        if let Some(max_props) = schema.get("maxProperties").and_then(|v| v.as_u64()) {
            rules.max_properties = Some(max_props as usize);
        }

        // Extract enum validation
        if let Some(enum_values) = schema.get("enum").and_then(|v| v.as_array()) {
            rules.allowed_values = Some(enum_values.clone());
        }

        // Extract Kubernetes-specific extensions
        Self::extract_k8s_extensions(&mut rules, schema);

        rules
    }

    /// Extract validation rules and create contract rules for complex constraints
    pub fn extract_contract_rules(schema: &Value) -> Vec<ContractRule> {
        let mut contracts = Vec::new();

        // Handle oneOf constraints
        if let Some(one_of) = schema.get("oneOf").and_then(|v| v.as_array()) {
            if !one_of.is_empty() {
                contracts.push(ContractRule {
                    name: "oneOf".to_string(),
                    expression: Self::generate_one_of_expression(one_of),
                    description: Some(
                        "Value must match exactly one of the specified schemas".to_string(),
                    ),
                    error_message: Some(
                        "Value does not match any of the allowed schemas".to_string(),
                    ),
                });
            }
        }

        // Handle anyOf constraints
        if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
            if !any_of.is_empty() {
                contracts.push(ContractRule {
                    name: "anyOf".to_string(),
                    expression: Self::generate_any_of_expression(any_of),
                    description: Some(
                        "Value must match at least one of the specified schemas".to_string(),
                    ),
                    error_message: Some(
                        "Value does not match any of the allowed schemas".to_string(),
                    ),
                });
            }
        }

        // Handle allOf constraints
        if let Some(all_of) = schema.get("allOf").and_then(|v| v.as_array()) {
            if !all_of.is_empty() {
                contracts.push(ContractRule {
                    name: "allOf".to_string(),
                    expression: Self::generate_all_of_expression(all_of),
                    description: Some("Value must match all of the specified schemas".to_string()),
                    error_message: Some("Value does not match all required schemas".to_string()),
                });
            }
        }

        // Handle not constraints
        if let Some(not_schema) = schema.get("not") {
            contracts.push(ContractRule {
                name: "not".to_string(),
                expression: Self::generate_not_expression(not_schema),
                description: Some("Value must not match the specified schema".to_string()),
                error_message: Some("Value matches a forbidden schema".to_string()),
            });
        }

        // Handle custom validation based on properties
        if let Some(properties) = schema.get("properties") {
            if let Some(custom_contracts) = Self::extract_custom_contracts(properties) {
                contracts.extend(custom_contracts);
            }
        }

        contracts
    }

    /// Parse string format from JSON Schema format field
    fn parse_string_format(format: &str) -> Option<StringFormat> {
        match format {
            "date-time" => Some(StringFormat::DateTime),
            "date" => Some(StringFormat::Date),
            "time" => Some(StringFormat::Time),
            "email" => Some(StringFormat::Email),
            "hostname" => Some(StringFormat::Hostname),
            "ipv4" => Some(StringFormat::Ipv4),
            "ipv6" => Some(StringFormat::Ipv6),
            "uri" => Some(StringFormat::Uri),
            "uri-reference" => Some(StringFormat::UriReference),
            "uuid" => Some(StringFormat::Uuid),
            // Kubernetes-specific formats
            "dns1123-subdomain" => Some(StringFormat::Dns1123Subdomain),
            "dns1123-label" => Some(StringFormat::Dns1123Label),
            _ => Some(StringFormat::Custom(format.to_string())),
        }
    }

    /// Extract Kubernetes-specific validation extensions
    fn extract_k8s_extensions(rules: &mut ValidationRules, schema: &Value) {
        // Check for x-kubernetes-int-or-string
        if let Some(int_or_string) = schema
            .get("x-kubernetes-int-or-string")
            .and_then(|v| v.as_bool())
        {
            rules.k8s_int_or_string = int_or_string;
        }

        // Check for x-kubernetes-preserve-unknown-fields
        if let Some(preserve_unknown) = schema
            .get("x-kubernetes-preserve-unknown-fields")
            .and_then(|v| v.as_bool())
        {
            rules.k8s_preserve_unknown_fields = preserve_unknown;
        }

        // Check for x-kubernetes-embedded-resource
        if let Some(embedded_resource) = schema
            .get("x-kubernetes-embedded-resource")
            .and_then(|v| v.as_bool())
        {
            rules.k8s_embedded_resource = embedded_resource;
        }

        // Check for x-kubernetes-validations (CEL expressions)
        if let Some(validations) = schema
            .get("x-kubernetes-validations")
            .and_then(|v| v.as_array())
        {
            for validation in validations {
                if let Some(rule) = validation.get("rule").and_then(|v| v.as_str()) {
                    rules.k8s_cel_validations.push(rule.to_string());
                }
            }
        }
    }

    /// Generate Nickel expression for oneOf constraint
    /// Attempts to generate actual validation for simple cases, returns true for complex ones
    fn generate_one_of_expression(schemas: &[Value]) -> String {
        // Try to detect simple type unions (e.g., oneOf: [{type: string}, {type: integer}])
        let types: Vec<&str> = schemas
            .iter()
            .filter_map(|s| s.get("type").and_then(|t| t.as_str()))
            .collect();

        if types.len() == schemas.len() && !types.is_empty() {
            // All schemas are simple type constraints - generate type check
            let checks: Vec<String> = types
                .iter()
                .map(|t| match *t {
                    "string" => "std.is_string value".to_string(),
                    "integer" | "number" => "std.is_number value".to_string(),
                    "boolean" => "std.is_bool value".to_string(),
                    "array" => "std.is_array value".to_string(),
                    "object" => "std.is_record value".to_string(),
                    "null" => "value == null".to_string(),
                    _ => "true".to_string(),
                })
                .collect();
            return format!("({})", checks.join(" || "));
        }

        // Try to detect enum-based oneOf (e.g., oneOf with const values)
        let consts: Vec<String> = schemas
            .iter()
            .filter_map(|s| s.get("const"))
            .filter_map(|v| {
                if v.is_string() {
                    Some(format!("\"{}\"", v.as_str().unwrap()))
                } else if v.is_number() {
                    Some(v.to_string())
                } else if v.is_boolean() {
                    Some(v.to_string())
                } else {
                    None
                }
            })
            .collect();

        if consts.len() == schemas.len() && !consts.is_empty() {
            return format!("std.array.elem value [{}]", consts.join(", "));
        }

        // Complex oneOf - return true with a comment noting the limitation
        // This ensures valid Nickel code while documenting the limitation
        format!("true # oneOf with {} schemas - complex validation not yet supported", schemas.len())
    }

    /// Generate Nickel expression for anyOf constraint
    /// Attempts to generate actual validation for simple cases
    fn generate_any_of_expression(schemas: &[Value]) -> String {
        // Try to detect simple type unions
        let types: Vec<&str> = schemas
            .iter()
            .filter_map(|s| s.get("type").and_then(|t| t.as_str()))
            .collect();

        if types.len() == schemas.len() && !types.is_empty() {
            let checks: Vec<String> = types
                .iter()
                .map(|t| match *t {
                    "string" => "std.is_string value".to_string(),
                    "integer" | "number" => "std.is_number value".to_string(),
                    "boolean" => "std.is_bool value".to_string(),
                    "array" => "std.is_array value".to_string(),
                    "object" => "std.is_record value".to_string(),
                    "null" => "value == null".to_string(),
                    _ => "true".to_string(),
                })
                .collect();
            return format!("({})", checks.join(" || "));
        }

        // Complex anyOf
        format!("true # anyOf with {} schemas - complex validation not yet supported", schemas.len())
    }

    /// Generate Nickel expression for allOf constraint
    /// For allOf, the schemas are typically merged at parse time, but we generate validation for remaining cases
    fn generate_all_of_expression(schemas: &[Value]) -> String {
        // allOf is typically handled by merging schemas during parsing
        // For runtime validation, we'd need to validate against all schemas
        // This is complex, so for now return true
        format!("true # allOf with {} constraints - merged at parse time", schemas.len())
    }

    /// Generate Nickel expression for not constraint
    fn generate_not_expression(schema: &Value) -> String {
        // Try to handle simple "not" cases
        if let Some(t) = schema.get("type").and_then(|t| t.as_str()) {
            let check = match t {
                "string" => "std.is_string value",
                "integer" | "number" => "std.is_number value",
                "boolean" => "std.is_bool value",
                "array" => "std.is_array value",
                "object" => "std.is_record value",
                "null" => "value == null",
                _ => return "true # not constraint - type not recognized".to_string(),
            };
            return format!("!({})", check);
        }

        // Handle "not" with enum/const
        if let Some(const_val) = schema.get("const") {
            if const_val.is_string() {
                return format!("value != \"{}\"", const_val.as_str().unwrap());
            } else if const_val.is_number() {
                return format!("value != {}", const_val);
            }
        }

        // Complex not constraint
        "true # not constraint - complex validation not yet supported".to_string()
    }

    /// Extract custom contracts from specific property patterns
    fn extract_custom_contracts(_properties: &Value) -> Option<Vec<ContractRule>> {
        // This could be extended to detect specific patterns in K8s schemas
        // For example, detecting name/namespace patterns, label validation, etc.
        None
    }

    /// Check if a schema has any validation rules worth extracting
    pub fn has_validation_rules(schema: &Value) -> bool {
        if let Some(obj) = schema.as_object() {
            // String validations
            obj.contains_key("minLength") ||
            obj.contains_key("maxLength") ||
            obj.contains_key("pattern") ||
            obj.contains_key("format") ||
            // Number validations
            obj.contains_key("minimum") ||
            obj.contains_key("maximum") ||
            obj.contains_key("exclusiveMinimum") ||
            obj.contains_key("exclusiveMaximum") ||
            // Array validations
            obj.contains_key("minItems") ||
            obj.contains_key("maxItems") ||
            obj.contains_key("uniqueItems") ||
            // Object validations
            obj.contains_key("minProperties") ||
            obj.contains_key("maxProperties") ||
            // Enum validation
            obj.contains_key("enum") ||
            // Complex constraints
            obj.contains_key("oneOf") ||
            obj.contains_key("anyOf") ||
            obj.contains_key("allOf") ||
            obj.contains_key("not") ||
            // Kubernetes extensions
            obj.contains_key("x-kubernetes-int-or-string") ||
            obj.contains_key("x-kubernetes-preserve-unknown-fields") ||
            obj.contains_key("x-kubernetes-embedded-resource") ||
            obj.contains_key("x-kubernetes-validations")
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_string_validation() {
        let schema = json!({
            "type": "string",
            "minLength": 1,
            "maxLength": 253,
            "pattern": "^[a-z0-9]([a-z0-9-]*[a-z0-9])?$",
            "format": "dns1123-subdomain"
        });

        let rules = ValidationExtractor::extract_validation_rules(&schema);

        assert_eq!(rules.min_length, Some(1));
        assert_eq!(rules.max_length, Some(253));
        assert_eq!(
            rules.pattern,
            Some("^[a-z0-9]([a-z0-9-]*[a-z0-9])?$".to_string())
        );
        assert_eq!(rules.format, Some(StringFormat::Dns1123Subdomain));
    }

    #[test]
    fn test_extract_number_validation() {
        let schema = json!({
            "type": "number",
            "minimum": 0,
            "maximum": 100,
            "exclusiveMinimum": -1
        });

        let rules = ValidationExtractor::extract_validation_rules(&schema);

        assert_eq!(rules.minimum, Some(0.0));
        assert_eq!(rules.maximum, Some(100.0));
        assert_eq!(rules.exclusive_minimum, Some(-1.0));
    }

    #[test]
    fn test_extract_array_validation() {
        let schema = json!({
            "type": "array",
            "minItems": 1,
            "maxItems": 10,
            "uniqueItems": true
        });

        let rules = ValidationExtractor::extract_validation_rules(&schema);

        assert_eq!(rules.min_items, Some(1));
        assert_eq!(rules.max_items, Some(10));
        assert_eq!(rules.unique_items, Some(true));
    }

    #[test]
    fn test_extract_k8s_extensions() {
        let schema = json!({
            "type": "string",
            "x-kubernetes-int-or-string": true,
            "x-kubernetes-preserve-unknown-fields": true,
            "x-kubernetes-validations": [
                {
                    "rule": "self.size() <= 1024",
                    "message": "Value too large"
                }
            ]
        });

        let rules = ValidationExtractor::extract_validation_rules(&schema);

        assert!(rules.k8s_int_or_string);
        assert!(rules.k8s_preserve_unknown_fields);
        assert_eq!(rules.k8s_cel_validations, vec!["self.size() <= 1024"]);
    }

    #[test]
    fn test_extract_enum_validation() {
        let schema = json!({
            "type": "string",
            "enum": ["Ready", "NotReady", "Unknown"]
        });

        let rules = ValidationExtractor::extract_validation_rules(&schema);

        assert!(rules.allowed_values.is_some());
        let values = rules.allowed_values.unwrap();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_has_validation_rules() {
        let schema_with_rules = json!({
            "type": "string",
            "minLength": 1
        });

        let schema_without_rules = json!({
            "type": "string"
        });

        assert!(ValidationExtractor::has_validation_rules(
            &schema_with_rules
        ));
        assert!(!ValidationExtractor::has_validation_rules(
            &schema_without_rules
        ));
    }

    #[test]
    fn test_extract_contract_rules() {
        let schema = json!({
            "oneOf": [
                {"type": "string"},
                {"type": "number"}
            ]
        });

        let contracts = ValidationExtractor::extract_contract_rules(&schema);

        assert_eq!(contracts.len(), 1);
        assert_eq!(contracts[0].name, "oneOf");
        assert!(contracts[0].description.is_some());
    }
}
