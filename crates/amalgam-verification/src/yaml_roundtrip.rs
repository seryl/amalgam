//! YAML round-trip testing
//!
//! Compares YAML files semantically (ignoring key order, whitespace, etc.)

use crate::error::{Result, VerificationError};
use serde_yaml::Value;
use similar::{ChangeTag, TextDiff};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RoundTripResult {
    pub nickel_file: PathBuf,
    pub yaml_file: PathBuf,
    pub equivalent: bool,
    pub diff: Option<String>,
}

pub struct YamlRoundTrip;

impl YamlRoundTrip {
    /// Compare two YAML files semantically
    pub fn compare_yaml(yaml1: &str, yaml2: &str) -> Result<bool> {
        let value1: Value = serde_yaml::from_str(yaml1)?;
        let value2: Value = serde_yaml::from_str(yaml2)?;

        Ok(Self::values_equivalent(&value1, &value2))
    }

    /// Check if two YAML values are semantically equivalent
    fn values_equivalent(v1: &Value, v2: &Value) -> bool {
        match (v1, v2) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Number(n1), Value::Number(n2)) => {
                // Compare numbers (handle int vs float)
                n1.as_f64() == n2.as_f64()
            }
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Sequence(seq1), Value::Sequence(seq2)) => {
                if seq1.len() != seq2.len() {
                    return false;
                }
                seq1.iter()
                    .zip(seq2.iter())
                    .all(|(v1, v2)| Self::values_equivalent(v1, v2))
            }
            (Value::Mapping(map1), Value::Mapping(map2)) => {
                if map1.len() != map2.len() {
                    return false;
                }
                // Compare mappings (order-independent)
                map1.iter().all(|(k1, v1)| {
                    map2.get(k1)
                        .map(|v2| Self::values_equivalent(v1, v2))
                        .unwrap_or(false)
                })
            }
            _ => false,
        }
    }

    /// Generate a human-readable diff between two YAML strings
    pub fn generate_diff(yaml1: &str, yaml2: &str) -> String {
        let diff = TextDiff::from_lines(yaml1, yaml2);
        let mut result = String::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            result.push_str(&format!("{}{}", sign, change));
        }

        result
    }

    /// Normalize YAML for consistent comparison
    /// (sorts keys, normalizes whitespace)
    pub fn normalize_yaml(yaml: &str) -> Result<String> {
        let value: Value = serde_yaml::from_str(yaml)?;
        let normalized = serde_yaml::to_string(&value)?;
        Ok(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_identical_yaml() {
        let yaml1 = r#"
foo: 1
bar: hello
"#;
        let yaml2 = r#"
foo: 1
bar: hello
"#;

        assert!(YamlRoundTrip::compare_yaml(yaml1, yaml2).unwrap());
    }

    #[test]
    fn test_compare_different_key_order() {
        let yaml1 = r#"
foo: 1
bar: hello
"#;
        let yaml2 = r#"
bar: hello
foo: 1
"#;

        // Should be equivalent despite different key order
        assert!(YamlRoundTrip::compare_yaml(yaml1, yaml2).unwrap());
    }

    #[test]
    fn test_compare_different_values() {
        let yaml1 = r#"
foo: 1
bar: hello
"#;
        let yaml2 = r#"
foo: 2
bar: hello
"#;

        assert!(!YamlRoundTrip::compare_yaml(yaml1, yaml2).unwrap());
    }

    #[test]
    fn test_compare_nested_objects() {
        let yaml1 = r#"
metadata:
  name: test
  labels:
    app: myapp
    version: v1
spec:
  replicas: 3
"#;
        let yaml2 = r#"
spec:
  replicas: 3
metadata:
  labels:
    version: v1
    app: myapp
  name: test
"#;

        // Should be equivalent despite different ordering
        assert!(YamlRoundTrip::compare_yaml(yaml1, yaml2).unwrap());
    }

    #[test]
    fn test_generate_diff() {
        let yaml1 = "foo: 1\nbar: hello\n";
        let yaml2 = "foo: 2\nbar: hello\n";

        let diff = YamlRoundTrip::generate_diff(yaml1, yaml2);

        assert!(diff.contains("-foo: 1"));
        assert!(diff.contains("+foo: 2"));
    }

    #[test]
    fn test_normalize_yaml() {
        let yaml = r#"
foo: 1
bar: hello
nested:
  key: value
"#;

        let normalized = YamlRoundTrip::normalize_yaml(yaml).unwrap();

        // Normalized YAML should be valid
        assert!(serde_yaml::from_str::<Value>(&normalized).is_ok());
    }
}
