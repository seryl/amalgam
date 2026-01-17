//! Unified type system using algebraic data types

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Validation rules extracted from JSON Schema and other sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub struct ValidationRules {
    // String validation
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub format: Option<StringFormat>,

    // Number validation
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
    pub exclusive_maximum: Option<f64>,

    // Array validation
    pub min_items: Option<usize>,
    pub max_items: Option<usize>,
    pub unique_items: Option<bool>,

    // Object validation
    pub min_properties: Option<usize>,
    pub max_properties: Option<usize>,

    // Enum validation
    pub allowed_values: Option<Vec<serde_json::Value>>,

    // K8s extensions
    pub k8s_int_or_string: bool,
    pub k8s_preserve_unknown_fields: bool,
    pub k8s_embedded_resource: bool,
    pub k8s_cel_validations: Vec<String>,
}


impl ValidationRules {
    /// Check if this has any validation rules
    pub fn is_empty(&self) -> bool {
        self.min_length.is_none()
            && self.max_length.is_none()
            && self.pattern.is_none()
            && self.format.is_none()
            && self.minimum.is_none()
            && self.maximum.is_none()
            && self.exclusive_minimum.is_none()
            && self.exclusive_maximum.is_none()
            && self.min_items.is_none()
            && self.max_items.is_none()
            && self.unique_items.is_none()
            && self.min_properties.is_none()
            && self.max_properties.is_none()
            && self.allowed_values.is_none()
            && !self.k8s_int_or_string
            && !self.k8s_preserve_unknown_fields
            && !self.k8s_embedded_resource
            && self.k8s_cel_validations.is_empty()
    }

    /// Create validation rules for string constraints
    pub fn string_constraints(
        min_len: Option<usize>,
        max_len: Option<usize>,
        pattern: Option<String>,
    ) -> Self {
        Self {
            min_length: min_len,
            max_length: max_len,
            pattern,
            ..Default::default()
        }
    }

    /// Create validation rules for number constraints
    pub fn number_constraints(min: Option<f64>, max: Option<f64>) -> Self {
        Self {
            minimum: min,
            maximum: max,
            ..Default::default()
        }
    }

    /// Create validation rules for enum values
    pub fn enum_values(values: Vec<serde_json::Value>) -> Self {
        Self {
            allowed_values: Some(values),
            ..Default::default()
        }
    }

    /// Mark as IntOrString type
    pub fn int_or_string() -> Self {
        Self {
            k8s_int_or_string: true,
            ..Default::default()
        }
    }

    /// Merge two validation rules, taking the most restrictive constraints
    ///
    /// This is used when combining allOf types where both schemas may have
    /// validation rules that should both apply.
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            // String validation - take most restrictive
            min_length: max_option(self.min_length, other.min_length),
            max_length: min_option(self.max_length, other.max_length),
            pattern: merge_patterns(&self.pattern, &other.pattern),
            format: self.format.clone().or_else(|| other.format.clone()),

            // Number validation - take most restrictive
            minimum: max_f64_option(self.minimum, other.minimum),
            maximum: min_f64_option(self.maximum, other.maximum),
            exclusive_minimum: max_f64_option(self.exclusive_minimum, other.exclusive_minimum),
            exclusive_maximum: min_f64_option(self.exclusive_maximum, other.exclusive_maximum),

            // Array validation - take most restrictive
            min_items: max_option(self.min_items, other.min_items),
            max_items: min_option(self.max_items, other.max_items),
            unique_items: match (self.unique_items, other.unique_items) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => self.unique_items.or(other.unique_items),
            },

            // Object validation - take most restrictive
            min_properties: max_option(self.min_properties, other.min_properties),
            max_properties: min_option(self.max_properties, other.max_properties),

            // Enum validation - intersection of allowed values
            allowed_values: merge_allowed_values(&self.allowed_values, &other.allowed_values),

            // K8s extensions - OR together (if either requires it, the merged type does)
            k8s_int_or_string: self.k8s_int_or_string || other.k8s_int_or_string,
            k8s_preserve_unknown_fields: self.k8s_preserve_unknown_fields
                || other.k8s_preserve_unknown_fields,
            k8s_embedded_resource: self.k8s_embedded_resource || other.k8s_embedded_resource,
            // Combine CEL validations from both
            k8s_cel_validations: {
                let mut validations = self.k8s_cel_validations.clone();
                for cel in &other.k8s_cel_validations {
                    if !validations.contains(cel) {
                        validations.push(cel.clone());
                    }
                }
                validations
            },
        }
    }
}

/// Helper: return the maximum of two Option<usize> values
fn max_option(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Helper: return the minimum of two Option<usize> values
fn min_option(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Helper: return the maximum of two Option<f64> values
fn max_f64_option(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Helper: return the minimum of two Option<f64> values
fn min_f64_option(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

/// Helper: merge two regex patterns into one that matches both
fn merge_patterns(a: &Option<String>, b: &Option<String>) -> Option<String> {
    match (a, b) {
        (Some(a), Some(b)) if a != b => {
            // Combine patterns with AND semantics using lookahead
            // This creates a pattern that requires both to match
            Some(format!("(?={}$)(?={}$).*", a, b))
        }
        (Some(a), Some(_)) => Some(a.clone()), // Same pattern
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (None, None) => None,
    }
}

/// Helper: merge allowed values by taking intersection
fn merge_allowed_values(
    a: &Option<Vec<serde_json::Value>>,
    b: &Option<Vec<serde_json::Value>>,
) -> Option<Vec<serde_json::Value>> {
    match (a, b) {
        (Some(a), Some(b)) => {
            // Intersection of allowed values
            let intersection: Vec<_> = a.iter().filter(|v| b.contains(v)).cloned().collect();
            if intersection.is_empty() {
                // No intersection - this would be unsatisfiable, but keep one set
                Some(a.clone())
            } else {
                Some(intersection)
            }
        }
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (None, None) => None,
    }
}

/// String format types from JSON Schema and Kubernetes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StringFormat {
    // JSON Schema standard formats
    DateTime,
    Date,
    Time,
    Email,
    Hostname,
    Ipv4,
    Ipv6,
    Uri,
    UriReference,
    Uuid,
    // Kubernetes-specific formats
    Dns1123Subdomain,
    Dns1123Label,
    LabelKey,
    LabelValue,
    // Custom format
    Custom(String),
}

/// A contract rule that can be applied to validate or transform values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractRule {
    pub name: String,
    pub expression: String,
    pub description: Option<String>,
    pub error_message: Option<String>,
}

impl ContractRule {
    /// Merge two lists of contracts, avoiding duplicates
    pub fn merge_contracts(a: &[ContractRule], b: &[ContractRule]) -> Vec<ContractRule> {
        let mut result = a.to_vec();
        for contract in b {
            // Avoid adding duplicates based on name
            if !result.iter().any(|c| c.name == contract.name) {
                result.push(contract.clone());
            }
        }
        result
    }
}

/// Hint for how to handle union types in target languages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnionCoercion {
    /// Prefer string representation (e.g., for IntOrString)
    PreferString,
    /// Prefer numeric representation
    PreferNumber,
    /// No preference - generate actual union
    NoPreference,
    /// Custom handler
    Custom(String),
}

/// Core type representation - algebraic data types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Primitive types
    String,
    Number,
    Integer,
    Bool,
    Null,
    Any,

    /// Compound types
    Array(Box<Type>),
    Map {
        key: Box<Type>,
        value: Box<Type>,
    },
    Optional(Box<Type>),

    /// Product type (struct/record)
    Record {
        fields: BTreeMap<String, Field>,
        open: bool, // Whether additional fields are allowed
    },

    /// Sum type (enum/union) with optional coercion hint
    Union {
        types: Vec<Type>,
        /// Hint for how to handle this union in target languages
        coercion_hint: Option<UnionCoercion>,
    },

    /// Tagged union (discriminated)
    TaggedUnion {
        tag_field: String,
        variants: BTreeMap<String, Type>,
    },

    /// Reference to another type with optional module information
    Reference {
        name: String,
        /// Full module path if this is a cross-module reference
        /// e.g., "io.k8s.api.core.v1" for NodeSelector
        module: Option<String>,
    },

    /// Type with validation constraints
    Constrained {
        base_type: Box<Type>,
        constraints: ValidationRules,
    },

    /// Contract/refinement type with structured rules
    Contract {
        base: Box<Type>,
        rules: Vec<ContractRule>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub ty: Type,
    pub required: bool,
    pub description: Option<String>,
    pub default: Option<serde_json::Value>,
    /// Validation rules extracted from schema
    pub validation: Option<ValidationRules>,
    /// Additional contract rules for this field
    pub contracts: Vec<ContractRule>,
}

impl Field {
    /// Merge two fields when combining allOf types
    ///
    /// This handles the case where the same field appears in multiple schemas
    /// that are being combined. The types may be merged into a union if they
    /// differ, but validation rules and contracts are always preserved and merged.
    pub fn merge_for_allof(existing: &Field, new: &Field) -> Field {
        // Determine the merged type
        let merged_ty = if existing.ty == new.ty {
            existing.ty.clone()
        } else {
            // Types differ - create a union
            Type::Union {
                types: vec![existing.ty.clone(), new.ty.clone()],
                coercion_hint: None,
            }
        };

        // Merge validation rules
        let merged_validation = match (&existing.validation, &new.validation) {
            (Some(a), Some(b)) => Some(a.merge(b)),
            (Some(a), None) => Some(a.clone()),
            (None, Some(b)) => Some(b.clone()),
            (None, None) => None,
        };

        // Merge contracts
        let merged_contracts = ContractRule::merge_contracts(&existing.contracts, &new.contracts);

        Field {
            ty: merged_ty,
            // For allOf, field is required only if required in ALL schemas
            required: existing.required && new.required,
            // Prefer description from new, fall back to existing
            description: new.description.clone().or_else(|| existing.description.clone()),
            // Prefer default from new, fall back to existing
            default: new.default.clone().or_else(|| existing.default.clone()),
            validation: merged_validation,
            contracts: merged_contracts,
        }
    }
}

/// Type system operations
pub struct TypeSystem {
    types: BTreeMap<String, Type>,
}

impl TypeSystem {
    pub fn new() -> Self {
        Self {
            types: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, name: String, ty: Type) {
        self.types.insert(name, ty);
    }

    pub fn resolve(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    pub fn is_compatible(&self, source: &Type, target: &Type) -> bool {
        match (source, target) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Null, Type::Optional(_)) => true,
            (s, Type::Optional(t)) => self.is_compatible(s, t),
            (Type::Integer, Type::Number) => true,
            (Type::Reference { name: s, .. }, t) => {
                if let Some(resolved) = self.resolve(s) {
                    self.is_compatible(resolved, t)
                } else {
                    false
                }
            }
            (s, Type::Reference { name: t, .. }) => {
                if let Some(resolved) = self.resolve(t) {
                    self.is_compatible(s, resolved)
                } else {
                    false
                }
            }
            (Type::Array(s), Type::Array(t)) => self.is_compatible(s, t),
            (Type::Union { types, .. }, t) => types.iter().all(|v| self.is_compatible(v, t)),
            (s, Type::Union { types, .. }) => types.iter().any(|v| self.is_compatible(s, v)),
            _ => source == target,
        }
    }
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_compatibility() {
        let mut ts = TypeSystem::new();

        // Register a custom type
        ts.register("MyString".to_string(), Type::String);

        // Test basic compatibility
        assert!(ts.is_compatible(&Type::String, &Type::String));
        assert!(ts.is_compatible(&Type::Integer, &Type::Number));
        assert!(ts.is_compatible(&Type::Null, &Type::Optional(Box::new(Type::String))));

        // Test reference resolution
        assert!(ts.is_compatible(
            &Type::Reference {
                name: "MyString".to_string(),
                module: None
            },
            &Type::String
        ));

        // Test union types
        let union = Type::Union {
            types: vec![Type::String, Type::Number],
            coercion_hint: None,
        };
        assert!(ts.is_compatible(&Type::String, &union));
        assert!(ts.is_compatible(&Type::Number, &union));
        assert!(!ts.is_compatible(&Type::Bool, &union));
    }

    #[test]
    fn test_validation_rules_merge_string_constraints() {
        let a = ValidationRules {
            min_length: Some(5),
            max_length: Some(100),
            pattern: Some("^[a-z]+$".to_string()),
            ..Default::default()
        };
        let b = ValidationRules {
            min_length: Some(10), // More restrictive
            max_length: Some(50), // More restrictive
            pattern: Some("^[a-z0-9]+$".to_string()),
            ..Default::default()
        };

        let merged = a.merge(&b);
        assert_eq!(merged.min_length, Some(10)); // Max of minimums
        assert_eq!(merged.max_length, Some(50)); // Min of maximums
        assert!(merged.pattern.is_some()); // Patterns should be merged
    }

    #[test]
    fn test_validation_rules_merge_number_constraints() {
        let a = ValidationRules {
            minimum: Some(0.0),
            maximum: Some(100.0),
            ..Default::default()
        };
        let b = ValidationRules {
            minimum: Some(10.0),
            maximum: Some(50.0),
            ..Default::default()
        };

        let merged = a.merge(&b);
        assert_eq!(merged.minimum, Some(10.0)); // Max of minimums
        assert_eq!(merged.maximum, Some(50.0)); // Min of maximums
    }

    #[test]
    fn test_validation_rules_merge_k8s_extensions() {
        let a = ValidationRules {
            k8s_int_or_string: true,
            k8s_cel_validations: vec!["self.size() > 0".to_string()],
            ..Default::default()
        };
        let b = ValidationRules {
            k8s_preserve_unknown_fields: true,
            k8s_cel_validations: vec!["self.size() < 100".to_string()],
            ..Default::default()
        };

        let merged = a.merge(&b);
        assert!(merged.k8s_int_or_string);
        assert!(merged.k8s_preserve_unknown_fields);
        assert_eq!(merged.k8s_cel_validations.len(), 2);
    }

    #[test]
    fn test_field_merge_for_allof() {
        let field_a = Field {
            ty: Type::String,
            required: true,
            description: Some("Field A".to_string()),
            default: None,
            validation: Some(ValidationRules {
                min_length: Some(5),
                ..Default::default()
            }),
            contracts: vec![ContractRule {
                name: "check_a".to_string(),
                expression: "self.len() > 0".to_string(),
                description: None,
                error_message: None,
            }],
        };

        let field_b = Field {
            ty: Type::String,
            required: true,
            description: Some("Field B".to_string()),
            default: None,
            validation: Some(ValidationRules {
                max_length: Some(100),
                ..Default::default()
            }),
            contracts: vec![ContractRule {
                name: "check_b".to_string(),
                expression: "self.len() < 200".to_string(),
                description: None,
                error_message: None,
            }],
        };

        let merged = Field::merge_for_allof(&field_a, &field_b);

        // Same types should not create a union
        assert_eq!(merged.ty, Type::String);
        assert!(merged.required);

        // Validation rules should be merged
        let validation = merged.validation.unwrap();
        assert_eq!(validation.min_length, Some(5));
        assert_eq!(validation.max_length, Some(100));

        // Contracts should be combined
        assert_eq!(merged.contracts.len(), 2);
    }

    #[test]
    fn test_field_merge_different_types() {
        let field_a = Field {
            ty: Type::String,
            required: true,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        };

        let field_b = Field {
            ty: Type::Integer,
            required: false,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        };

        let merged = Field::merge_for_allof(&field_a, &field_b);

        // Different types should create a union
        match merged.ty {
            Type::Union { types, .. } => {
                assert_eq!(types.len(), 2);
            }
            _ => panic!("Expected union type"),
        }

        // Required should be AND (true && false = false)
        assert!(!merged.required);
    }

    #[test]
    fn test_contract_rule_merge() {
        let contracts_a = vec![
            ContractRule {
                name: "rule1".to_string(),
                expression: "expr1".to_string(),
                description: None,
                error_message: None,
            },
            ContractRule {
                name: "rule2".to_string(),
                expression: "expr2".to_string(),
                description: None,
                error_message: None,
            },
        ];

        let contracts_b = vec![
            ContractRule {
                name: "rule2".to_string(), // Duplicate
                expression: "expr2_new".to_string(),
                description: None,
                error_message: None,
            },
            ContractRule {
                name: "rule3".to_string(),
                expression: "expr3".to_string(),
                description: None,
                error_message: None,
            },
        ];

        let merged = ContractRule::merge_contracts(&contracts_a, &contracts_b);

        // Should have 3 rules (rule2 deduped by name)
        assert_eq!(merged.len(), 3);
        assert!(merged.iter().any(|c| c.name == "rule1"));
        assert!(merged.iter().any(|c| c.name == "rule2"));
        assert!(merged.iter().any(|c| c.name == "rule3"));
    }
}
