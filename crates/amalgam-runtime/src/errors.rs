//! Validation error types for amalgam-runtime.
//!
//! This module provides structured error types for validation failures,
//! including JSON path support for locating errors within nested structures.

use std::fmt;

/// A collection of validation errors.
///
/// When validating a configuration, multiple errors may be found.
/// This type collects all errors found during validation.
#[derive(Debug, Clone, Default)]
pub struct ValidationErrors {
    /// Individual validation errors
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create an empty validation errors collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a validation errors collection from a single error.
    pub fn from_error(error: ValidationError) -> Self {
        Self {
            errors: vec![error],
        }
    }

    /// Add an error to the collection.
    pub fn push(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Check if there are any errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Merge another ValidationErrors into this one.
    pub fn merge(&mut self, other: ValidationErrors) {
        self.errors.extend(other.errors);
    }

    /// Prefix all error paths with a given path segment.
    ///
    /// This is useful when validating nested structures.
    pub fn with_path_prefix(mut self, prefix: &str) -> Self {
        for error in &mut self.errors {
            if error.path.is_empty() {
                error.path = prefix.to_string();
            } else if error.path.starts_with('[') {
                error.path = format!("{}{}", prefix, error.path);
            } else {
                error.path = format!("{}.{}", prefix, error.path);
            }
        }
        self
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            write!(f, "No validation errors")
        } else if self.errors.len() == 1 {
            write!(f, "Validation error: {}", self.errors[0])
        } else {
            writeln!(f, "{} validation errors:", self.errors.len())?;
            for (i, error) in self.errors.iter().enumerate() {
                writeln!(f, "  {}. {}", i + 1, error)?;
            }
            Ok(())
        }
    }
}

impl std::error::Error for ValidationErrors {}

impl From<ValidationError> for ValidationErrors {
    fn from(error: ValidationError) -> Self {
        Self::from_error(error)
    }
}

impl IntoIterator for ValidationErrors {
    type Item = ValidationError;
    type IntoIter = std::vec::IntoIter<ValidationError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a> IntoIterator for &'a ValidationErrors {
    type Item = &'a ValidationError;
    type IntoIter = std::slice::Iter<'a, ValidationError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

/// A single validation error with location and context.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// JSON path to the field that failed validation.
    ///
    /// Examples:
    /// - `""` - root object
    /// - `"spec"` - the spec field
    /// - `"spec.replicas"` - nested field
    /// - `"spec.containers[0].image"` - array element field
    pub path: String,

    /// Human-readable error message.
    pub message: String,

    /// The validation rule that was violated (if known).
    ///
    /// Examples:
    /// - `"minLength"` - string too short
    /// - `"maximum"` - number too large
    /// - `"required"` - required field missing
    /// - `"pattern"` - regex pattern mismatch
    /// - `"contract:PositiveInt"` - Nickel contract violation
    pub rule: Option<String>,

    /// The expected value or constraint (if applicable).
    pub expected: Option<String>,

    /// The actual value that failed validation (if applicable).
    pub actual: Option<String>,
}

impl ValidationError {
    /// Create a new validation error.
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            rule: None,
            expected: None,
            actual: None,
        }
    }

    /// Set the rule that was violated.
    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rule = Some(rule.into());
        self
    }

    /// Set the expected value.
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    /// Set the actual value.
    pub fn with_actual(mut self, actual: impl Into<String>) -> Self {
        self.actual = Some(actual.into());
        self
    }

    // --- Common error constructors ---

    /// Create an error for a required field that is missing.
    pub fn required(path: impl Into<String>) -> Self {
        Self::new(path, "required field is missing").with_rule("required")
    }

    /// Create an error for a string that is too short.
    pub fn min_length(path: impl Into<String>, min: usize, actual: usize) -> Self {
        Self::new(path, format!("string length {} is less than minimum {}", actual, min))
            .with_rule("minLength")
            .with_expected(format!(">= {}", min))
            .with_actual(actual.to_string())
    }

    /// Create an error for a string that is too long.
    pub fn max_length(path: impl Into<String>, max: usize, actual: usize) -> Self {
        Self::new(
            path,
            format!("string length {} exceeds maximum {}", actual, max),
        )
        .with_rule("maxLength")
        .with_expected(format!("<= {}", max))
        .with_actual(actual.to_string())
    }

    /// Create an error for a pattern mismatch.
    pub fn pattern(path: impl Into<String>, pattern: &str, value: &str) -> Self {
        Self::new(path, format!("value does not match pattern: {}", pattern))
            .with_rule("pattern")
            .with_expected(pattern.to_string())
            .with_actual(value.to_string())
    }

    /// Create an error for a number below minimum.
    pub fn minimum(path: impl Into<String>, min: f64, actual: f64) -> Self {
        Self::new(path, format!("value {} is less than minimum {}", actual, min))
            .with_rule("minimum")
            .with_expected(format!(">= {}", min))
            .with_actual(actual.to_string())
    }

    /// Create an error for a number above maximum.
    pub fn maximum(path: impl Into<String>, max: f64, actual: f64) -> Self {
        Self::new(path, format!("value {} exceeds maximum {}", actual, max))
            .with_rule("maximum")
            .with_expected(format!("<= {}", max))
            .with_actual(actual.to_string())
    }

    /// Create an error for an invalid enum value.
    pub fn invalid_enum(path: impl Into<String>, allowed: &[String], actual: &str) -> Self {
        Self::new(
            path,
            format!(
                "invalid value '{}', must be one of: {}",
                actual,
                allowed.join(", ")
            ),
        )
        .with_rule("enum")
        .with_expected(allowed.join(" | "))
        .with_actual(actual.to_string())
    }

    /// Create an error for a Nickel contract violation.
    pub fn contract(path: impl Into<String>, contract_name: &str, message: impl Into<String>) -> Self {
        Self::new(path, message).with_rule(format!("contract:{}", contract_name))
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.message)?;
        } else {
            write!(f, "{}: {}", self.path, self.message)?;
        }

        if let Some(rule) = &self.rule {
            write!(f, " [{}]", rule)?;
        }

        Ok(())
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::new("spec.replicas", "value must be positive")
            .with_rule("minimum")
            .with_expected("> 0")
            .with_actual("-1");

        let display = format!("{}", error);
        assert!(display.contains("spec.replicas"));
        assert!(display.contains("value must be positive"));
        assert!(display.contains("[minimum]"));
    }

    #[test]
    fn test_validation_errors_path_prefix() {
        let mut errors = ValidationErrors::new();
        errors.push(ValidationError::new("name", "too short"));
        errors.push(ValidationError::new("labels[0]", "invalid"));

        let prefixed = errors.with_path_prefix("metadata");

        assert_eq!(prefixed.errors[0].path, "metadata.name");
        assert_eq!(prefixed.errors[1].path, "metadata.labels[0]");
    }

    #[test]
    fn test_common_error_constructors() {
        let required = ValidationError::required("spec.image");
        assert_eq!(required.rule, Some("required".to_string()));

        let min_len = ValidationError::min_length("metadata.name", 3, 1);
        assert!(min_len.message.contains("1"));
        assert!(min_len.message.contains("3"));

        let pattern = ValidationError::pattern("metadata.name", "^[a-z]+$", "Test123");
        assert!(pattern.message.contains("pattern"));
    }
}
