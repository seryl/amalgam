//! Runtime types and contracts for Kubernetes-style schemas
//! 
//! This module provides common runtime types like IntOrString, Quantity, etc.
//! that are used across Kubernetes and CRD schemas.

use amalgam_core::types::Type;
use std::collections::HashMap;

/// Common runtime types that need special handling
#[derive(Debug, Clone)]
pub struct RuntimeTypes {
    types: HashMap<String, RuntimeType>,
}

#[derive(Debug, Clone)]
pub struct RuntimeType {
    pub name: String,
    pub nickel_type: String,
    pub contract: Option<String>,
    pub description: String,
}

impl RuntimeTypes {
    pub fn kubernetes() -> Self {
        let mut types = HashMap::new();
        
        // IntOrString - can be either an integer or a string
        types.insert(
            "IntOrString".to_string(),
            RuntimeType {
                name: "IntOrString".to_string(),
                nickel_type: "String".to_string(),
                contract: Some(r#"fun value =>
      std.is_string value || std.is_number value"#.to_string()),
                description: "A type that can be either an Int or a String".to_string(),
            }
        );
        
        // Quantity - resource quantities like "100m" or "2Gi"
        types.insert(
            "Quantity".to_string(),
            RuntimeType {
                name: "Quantity".to_string(),
                nickel_type: "String".to_string(),
                contract: Some(r#"fun value =>
      std.is_string value && 
      std.string.match value "^[0-9]+(\\.[0-9]+)?([EPTGMK]i?|m)?$" != null"#.to_string()),
                description: "A fixed-point representation of a resource quantity".to_string(),
            }
        );
        
        // RawExtension - arbitrary JSON/YAML data
        types.insert(
            "RawExtension".to_string(),
            RuntimeType {
                name: "RawExtension".to_string(),
                nickel_type: "Dyn".to_string(),
                contract: None,  // Dyn already accepts anything
                description: "Raw JSON/YAML data that can contain any value".to_string(),
            }
        );
        
        // Time - RFC3339 timestamp
        // Format: 2006-01-02T15:04:05Z or 2006-01-02T15:04:05+07:00
        types.insert(
            "Time".to_string(),
            RuntimeType {
                name: "Time".to_string(),
                nickel_type: "String".to_string(),
                contract: Some(r#"fun value =>
      std.is_string value &&
      std.string.match value "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(Z|[+-][0-9]{2}:[0-9]{2})$" != null"#.to_string()),
                description: "A timestamp in RFC3339 format".to_string(),
            }
        );

        // MicroTime - RFC3339 timestamp with microsecond precision
        // Format: 2006-01-02T15:04:05.000000Z or 2006-01-02T15:04:05.000000+07:00
        types.insert(
            "MicroTime".to_string(),
            RuntimeType {
                name: "MicroTime".to_string(),
                nickel_type: "String".to_string(),
                contract: Some(r#"fun value =>
      std.is_string value &&
      std.string.match value "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(\\.[0-9]{1,9})?(Z|[+-][0-9]{2}:[0-9]{2})$" != null"#.to_string()),
                description: "A timestamp with microsecond precision".to_string(),
            }
        );
        
        // Duration - time duration like "30s" or "2h45m"
        types.insert(
            "Duration".to_string(),
            RuntimeType {
                name: "Duration".to_string(),
                nickel_type: "String".to_string(),
                contract: Some(r#"fun value =>
      std.is_string value && 
      std.string.match value "^[0-9]+(ns|us|µs|ms|s|m|h)$" != null"#.to_string()),
                description: "A time duration".to_string(),
            }
        );
        
        RuntimeTypes { types }
    }
    
    /// Check if a type name is a known runtime type
    pub fn is_runtime_type(&self, name: &str) -> bool {
        // Handle both short names (IntOrString) and qualified names
        let short_name = name.split('.').last().unwrap_or(name);
        self.types.contains_key(short_name)
    }
    
    /// Get the runtime type definition
    pub fn get(&self, name: &str) -> Option<&RuntimeType> {
        let short_name = name.split('.').last().unwrap_or(name);
        self.types.get(short_name)
    }
    
    /// Generate the v0.ncl file content with all runtime types
    pub fn generate_v0_module(&self) -> String {
        let mut content = String::from(r#"# Unversioned runtime types for Kubernetes
# These types are used across multiple API versions

{
"#);
        
        // Add type definitions
        for (name, rt) in &self.types {
            content.push_str(&format!(
                "  # {}\n  {} = {},\n\n",
                rt.description,
                name,
                rt.nickel_type
            ));
        }
        
        // Add contracts section
        content.push_str("  # Runtime validation contracts\n");
        content.push_str("  contracts = {\n");
        
        for (name, rt) in &self.types {
            if let Some(contract) = &rt.contract {
                let contract_name = name.chars().next().unwrap().to_lowercase().to_string() 
                    + &name[1..];
                content.push_str(&format!(
                    "    # Contract for {}\n    {} = {},\n\n",
                    name,
                    contract_name,
                    contract.replace('\n', "\n    ")
                ));
            }
        }
        
        content.push_str("  },\n}\n");
        content
    }
    
    /// Transform a Type::Reference to use runtime types when appropriate
    pub fn transform_type(&self, typ: &Type) -> Type {
        match typ {
            Type::Reference { name, module: _ } => {
                if self.is_runtime_type(name) {
                    // Transform to reference the v0 module
                    Type::Reference {
                        name: name.split('.').last().unwrap_or(name).to_string(),
                        module: Some("k8s.io.v0".to_string()),
                    }
                } else {
                    typ.clone()
                }
            }
            Type::Optional(inner) => Type::Optional(Box::new(self.transform_type(inner))),
            Type::Array(inner) => Type::Array(Box::new(self.transform_type(inner))),
            _ => typ.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_types() {
        let rt = RuntimeTypes::kubernetes();

        assert!(rt.is_runtime_type("IntOrString"));
        assert!(rt.is_runtime_type("io.k8s.apimachinery.pkg.util.intstr.IntOrString"));

        let intorstring = rt.get("IntOrString").unwrap();
        assert_eq!(intorstring.nickel_type, "String");
        assert!(intorstring.contract.is_some());
    }

    #[test]
    fn test_v0_generation() {
        let rt = RuntimeTypes::kubernetes();
        let content = rt.generate_v0_module();

        assert!(content.contains("IntOrString = String"));
        assert!(content.contains("contracts ="));
        assert!(content.contains("intOrString ="));
    }

    #[test]
    fn test_time_type_has_rfc3339_validation() {
        let rt = RuntimeTypes::kubernetes();
        let time = rt.get("Time").unwrap();

        assert!(time.contract.is_some());
        let contract = time.contract.as_ref().unwrap();

        // Should contain RFC3339 pattern
        assert!(contract.contains("std.string.match"));
        assert!(contract.contains("[0-9]{4}-[0-9]{2}-[0-9]{2}"));
        assert!(contract.contains("T[0-9]{2}:[0-9]{2}:[0-9]{2}"));
        assert!(contract.contains("Z|[+-][0-9]{2}:[0-9]{2}"));
    }

    #[test]
    fn test_microtime_type_has_rfc3339_validation() {
        let rt = RuntimeTypes::kubernetes();
        let microtime = rt.get("MicroTime").unwrap();

        assert!(microtime.contract.is_some());
        let contract = microtime.contract.as_ref().unwrap();

        // Should contain RFC3339 pattern with optional fractional seconds
        assert!(contract.contains("std.string.match"));
        assert!(contract.contains("[0-9]{4}-[0-9]{2}-[0-9]{2}"));
        assert!(contract.contains("(\\\\.[0-9]{1,9})?")); // Optional fractional seconds
        assert!(contract.contains("Z|[+-][0-9]{2}:[0-9]{2}"));
    }

    #[test]
    fn test_duration_type_has_validation() {
        let rt = RuntimeTypes::kubernetes();
        let duration = rt.get("Duration").unwrap();

        assert!(duration.contract.is_some());
        let contract = duration.contract.as_ref().unwrap();

        // Should contain duration pattern
        assert!(contract.contains("std.string.match"));
        assert!(contract.contains("ns|us|µs|ms|s|m|h"));
    }

    #[test]
    fn test_quantity_type_has_validation() {
        let rt = RuntimeTypes::kubernetes();
        let quantity = rt.get("Quantity").unwrap();

        assert!(quantity.contract.is_some());
        let contract = quantity.contract.as_ref().unwrap();

        // Should contain quantity pattern
        assert!(contract.contains("std.string.match"));
        assert!(contract.contains("[EPTGMK]i?|m")); // SI suffixes
    }
}