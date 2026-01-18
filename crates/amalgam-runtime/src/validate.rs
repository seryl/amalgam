//! Validation traits for amalgam-generated types.
//!
//! This module provides traits for validating configuration objects,
//! with support for both schema-based validation and Nickel contract evaluation.

use crate::errors::{ValidationError, ValidationErrors};

/// Trait for types that can be validated.
///
/// Generated types implement this trait with validation logic derived
/// from JSON Schema constraints and Nickel contracts.
///
/// # Example
///
/// ```rust,ignore
/// use amalgam_runtime::{Validate, ValidationContext};
///
/// let deployment = Deployment::new().with_name("app");
///
/// // Simple validation
/// deployment.validate()?;
///
/// // Validation with context (for cross-field validation)
/// let ctx = ValidationContext::new();
/// deployment.validate_with_context(&ctx)?;
/// ```
pub trait Validate {
    /// Validate the value against its schema constraints.
    ///
    /// Returns `Ok(())` if valid, or `Err(ValidationErrors)` with all violations.
    fn validate(&self) -> Result<(), ValidationErrors>;

    /// Validate with additional context.
    ///
    /// The context can be used for cross-field validation and tracking the current path.
    fn validate_with_context(&self, _ctx: &ValidationContext) -> Result<(), ValidationErrors> {
        // Default implementation ignores context
        self.validate()
    }
}

/// Context for validation operations.
///
/// This provides:
/// - Path tracking for nested validation
/// - Cross-field validation support
/// - Custom validation state
#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    /// Current JSON path being validated
    path: Vec<String>,
    /// Whether to collect all errors or stop at first
    collect_all: bool,
    /// Custom state for cross-field validation
    state: ValidationState,
}

/// Custom state for cross-field validation.
#[derive(Debug, Clone, Default)]
pub struct ValidationState {
    /// Values seen during validation (for uniqueness checks, etc.)
    pub seen_values: std::collections::HashSet<String>,
    /// Custom key-value pairs for complex validation
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

impl ValidationContext {
    /// Create a new validation context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a context that collects all errors instead of stopping at first.
    pub fn collect_all() -> Self {
        Self {
            collect_all: true,
            ..Default::default()
        }
    }

    /// Get the current JSON path as a string.
    pub fn current_path(&self) -> String {
        self.path.join(".")
    }

    /// Push a path segment for nested validation.
    pub fn push_path(&mut self, segment: impl Into<String>) {
        self.path.push(segment.into());
    }

    /// Pop a path segment after nested validation.
    pub fn pop_path(&mut self) {
        self.path.pop();
    }

    /// Push an array index to the path.
    pub fn push_index(&mut self, index: usize) {
        self.path.push(format!("[{}]", index));
    }

    /// Create an error at the current path.
    pub fn error(&self, message: impl Into<String>) -> ValidationError {
        ValidationError::new(self.current_path(), message)
    }

    /// Check if we should collect all errors.
    pub fn should_collect_all(&self) -> bool {
        self.collect_all
    }

    /// Get mutable access to the validation state.
    pub fn state_mut(&mut self) -> &mut ValidationState {
        &mut self.state
    }

    /// Get read access to the validation state.
    pub fn state(&self) -> &ValidationState {
        &self.state
    }
}

/// Extension trait for validating Option fields.
pub trait ValidateOption {
    /// Validate the inner value if present.
    fn validate_if_present(&self, ctx: &ValidationContext) -> Result<(), ValidationErrors>;
}

impl<T: Validate> ValidateOption for Option<T> {
    fn validate_if_present(&self, ctx: &ValidationContext) -> Result<(), ValidationErrors> {
        if let Some(inner) = self {
            inner.validate_with_context(ctx)
        } else {
            Ok(())
        }
    }
}

/// Extension trait for validating Vec fields.
pub trait ValidateVec {
    /// Validate each element in the vector.
    fn validate_elements(&self, ctx: &mut ValidationContext) -> Result<(), ValidationErrors>;
}

impl<T: Validate> ValidateVec for Vec<T> {
    fn validate_elements(&self, ctx: &mut ValidationContext) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        for (i, item) in self.iter().enumerate() {
            ctx.push_index(i);
            if let Err(e) = item.validate_with_context(ctx) {
                errors.merge(e);
                if !ctx.should_collect_all() {
                    ctx.pop_path();
                    return Err(errors);
                }
            }
            ctx.pop_path();
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// --- Validation helper functions ---

/// Validate that a string meets minimum length.
pub fn validate_min_length(value: &str, min: usize, path: &str) -> Result<(), ValidationError> {
    if value.len() < min {
        Err(ValidationError::min_length(path, min, value.len()))
    } else {
        Ok(())
    }
}

/// Validate that a string meets maximum length.
pub fn validate_max_length(value: &str, max: usize, path: &str) -> Result<(), ValidationError> {
    if value.len() > max {
        Err(ValidationError::max_length(path, max, value.len()))
    } else {
        Ok(())
    }
}

/// Validate that a string matches a pattern.
pub fn validate_pattern(value: &str, pattern: &str, path: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(pattern).map_err(|_| {
        ValidationError::new(path, format!("invalid regex pattern: {}", pattern))
    })?;

    if re.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::pattern(path, pattern, value))
    }
}

/// Validate that a number meets minimum value.
pub fn validate_minimum(value: f64, min: f64, path: &str) -> Result<(), ValidationError> {
    if value < min {
        Err(ValidationError::minimum(path, min, value))
    } else {
        Ok(())
    }
}

/// Validate that a number meets maximum value.
pub fn validate_maximum(value: f64, max: f64, path: &str) -> Result<(), ValidationError> {
    if value > max {
        Err(ValidationError::maximum(path, max, value))
    } else {
        Ok(())
    }
}

/// Validate that a value is in an allowed set.
pub fn validate_enum<T: ToString>(value: &T, allowed: &[&str], path: &str) -> Result<(), ValidationError> {
    let s = value.to_string();
    if allowed.contains(&s.as_str()) {
        Ok(())
    } else {
        Err(ValidationError::invalid_enum(
            path,
            &allowed.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            &s,
        ))
    }
}

// --- Default implementations for primitives ---

impl Validate for String {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(()) // No constraints by default
    }
}

impl Validate for bool {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

impl Validate for i32 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

impl Validate for i64 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

impl Validate for u32 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

impl Validate for u64 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

impl Validate for f32 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

impl Validate for f64 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

impl<T: Validate> Validate for Option<T> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if let Some(inner) = self {
            inner.validate()
        } else {
            Ok(())
        }
    }

    fn validate_with_context(&self, ctx: &ValidationContext) -> Result<(), ValidationErrors> {
        if let Some(inner) = self {
            inner.validate_with_context(ctx)
        } else {
            Ok(())
        }
    }
}

impl<T: Validate> Validate for Vec<T> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut ctx = ValidationContext::collect_all();
        self.validate_elements(&mut ctx)
    }

    fn validate_with_context(&self, ctx: &ValidationContext) -> Result<(), ValidationErrors> {
        let mut new_ctx = ctx.clone();
        self.validate_elements(&mut new_ctx)
    }
}

impl Validate for serde_json::Value {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(()) // JSON values are always structurally valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestStruct {
        name: Option<String>,
        count: Option<i32>,
    }

    impl Validate for TestStruct {
        fn validate(&self) -> Result<(), ValidationErrors> {
            let mut errors = ValidationErrors::new();

            // Name is required
            if self.name.is_none() {
                errors.push(ValidationError::required("name"));
            } else if let Some(ref name) = self.name {
                if name.len() < 3 {
                    errors.push(ValidationError::min_length("name", 3, name.len()));
                }
            }

            // Count must be positive if present
            if let Some(count) = self.count {
                if count < 0 {
                    errors.push(ValidationError::minimum("count", 0.0, count as f64));
                }
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }
    }

    #[test]
    fn test_validate_success() {
        let s = TestStruct {
            name: Some("test".into()),
            count: Some(5),
        };
        assert!(s.validate().is_ok());
    }

    #[test]
    fn test_validate_missing_required() {
        let s = TestStruct {
            name: None,
            count: Some(5),
        };
        let result = s.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.errors[0].message.contains("required"));
    }

    #[test]
    fn test_validate_multiple_errors() {
        let s = TestStruct {
            name: Some("ab".into()), // Too short
            count: Some(-1),          // Negative
        };
        let result = s.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_validation_context_path() {
        let mut ctx = ValidationContext::new();
        ctx.push_path("spec");
        ctx.push_path("containers");
        ctx.push_index(0);
        ctx.push_path("image");

        assert_eq!(ctx.current_path(), "spec.containers.[0].image");
    }

    #[test]
    fn test_validate_helpers() {
        assert!(validate_min_length("hello", 3, "field").is_ok());
        assert!(validate_min_length("hi", 3, "field").is_err());

        assert!(validate_maximum(5.0, 10.0, "field").is_ok());
        assert!(validate_maximum(15.0, 10.0, "field").is_err());

        assert!(validate_enum(&"Running", &["Pending", "Running", "Stopped"], "status").is_ok());
        assert!(validate_enum(&"Unknown", &["Pending", "Running", "Stopped"], "status").is_err());
    }
}
