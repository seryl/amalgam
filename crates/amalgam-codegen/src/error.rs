//! Error types and batch error reporting for code generation
//!
//! This module provides:
//! - Standard error types for code generation failures
//! - Batch error collection for comprehensive error reporting
//! - Integration with the validation system

use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Code generation error: {0}")]
    Generation(String),

    #[error("Unsupported type: {0}")]
    UnsupportedType(String),

    #[error("Invalid IR: {0}")]
    InvalidIR(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Fmt(#[from] std::fmt::Error),

    #[error("Batch errors ({count} total):\n{summary}")]
    Batch { count: usize, summary: String },
}

/// Location context for where an error occurred
#[derive(Debug, Clone, Default)]
pub struct ErrorLocation {
    /// Module name (e.g., "io.k8s.api.core.v1")
    pub module: Option<String>,
    /// Type name (e.g., "Pod")
    pub type_name: Option<String>,
    /// Field name (e.g., "metadata")
    pub field: Option<String>,
}

impl ErrorLocation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_module(mut self, module: impl Into<String>) -> Self {
        self.module = Some(module.into());
        self
    }

    pub fn in_type(mut self, type_name: impl Into<String>) -> Self {
        self.type_name = Some(type_name.into());
        self
    }

    pub fn in_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.module, &self.type_name, &self.field) {
            (Some(m), Some(t), Some(field)) => write!(f, "{}.{}::{}", m, t, field),
            (Some(m), Some(t), None) => write!(f, "{}.{}", m, t),
            (Some(m), None, Some(field)) => write!(f, "{}::{}", m, field),
            (Some(m), None, None) => write!(f, "{}", m),
            (None, Some(t), Some(field)) => write!(f, "{}::{}", t, field),
            (None, Some(t), None) => write!(f, "{}", t),
            (None, None, Some(field)) => write!(f, "::{}", field),
            (None, None, None) => write!(f, "<unknown location>"),
        }
    }
}

/// Category of error for better organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Missing type reference
    MissingType,
    /// Invalid module reference
    InvalidModule,
    /// Invalid type name
    InvalidTypeName,
    /// Import path calculation failed
    ImportPath,
    /// Type conversion error
    TypeConversion,
    /// Configuration error
    Configuration,
    /// Other/uncategorized
    Other,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::MissingType => write!(f, "MISSING_TYPE"),
            ErrorCategory::InvalidModule => write!(f, "INVALID_MODULE"),
            ErrorCategory::InvalidTypeName => write!(f, "INVALID_TYPE_NAME"),
            ErrorCategory::ImportPath => write!(f, "IMPORT_PATH"),
            ErrorCategory::TypeConversion => write!(f, "TYPE_CONVERSION"),
            ErrorCategory::Configuration => write!(f, "CONFIGURATION"),
            ErrorCategory::Other => write!(f, "OTHER"),
        }
    }
}

/// A single error entry in the batch
#[derive(Debug, Clone)]
pub struct ErrorEntry {
    /// Error category
    pub category: ErrorCategory,
    /// Error location
    pub location: ErrorLocation,
    /// Error message
    pub message: String,
    /// Optional suggestion for fixing
    pub suggestion: Option<String>,
}

impl ErrorEntry {
    pub fn new(category: ErrorCategory, message: impl Into<String>) -> Self {
        Self {
            category,
            location: ErrorLocation::new(),
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn at(mut self, location: ErrorLocation) -> Self {
        self.location = location;
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Create a missing type error
    pub fn missing_type(type_name: &str, context: &str) -> Self {
        Self::new(
            ErrorCategory::MissingType,
            format!("Type '{}' not found ({})", type_name, context),
        )
    }

    /// Create an invalid module error
    pub fn invalid_module(module: &str, reason: &str) -> Self {
        Self::new(
            ErrorCategory::InvalidModule,
            format!("Invalid module '{}': {}", module, reason),
        )
    }

    /// Create a type conversion error
    pub fn type_conversion(from: &str, reason: &str) -> Self {
        Self::new(
            ErrorCategory::TypeConversion,
            format!("Cannot convert type '{}': {}", from, reason),
        )
    }
}

impl fmt::Display for ErrorEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] at {}: {}", self.category, self.location, self.message)?;
        if let Some(ref suggestion) = self.suggestion {
            write!(f, " (suggestion: {})", suggestion)?;
        }
        Ok(())
    }
}

/// Batch error collector for accumulating errors during code generation
///
/// Instead of failing immediately on the first error, this collector
/// accumulates all errors so they can be reported together.
///
/// ## Usage
///
/// ```ignore
/// let mut errors = BatchErrors::new();
///
/// // During code generation...
/// if some_type_is_missing() {
///     errors.add(ErrorEntry::missing_type("SomeType", "referenced in Pod.spec"));
/// }
///
/// // At the end of generation...
/// if errors.has_errors() {
///     return Err(errors.into());
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct BatchErrors {
    entries: Vec<ErrorEntry>,
    /// Current context stack for location tracking
    module_context: Option<String>,
    type_context: Option<String>,
}

impl BatchErrors {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current module context for subsequent errors
    pub fn set_module_context(&mut self, module: impl Into<String>) {
        self.module_context = Some(module.into());
    }

    /// Set the current type context for subsequent errors
    pub fn set_type_context(&mut self, type_name: impl Into<String>) {
        self.type_context = Some(type_name.into());
    }

    /// Clear the type context (usually when moving to next type)
    pub fn clear_type_context(&mut self) {
        self.type_context = None;
    }

    /// Clear all context (usually when moving to next module)
    pub fn clear_context(&mut self) {
        self.module_context = None;
        self.type_context = None;
    }

    /// Add an error with the current context
    pub fn add(&mut self, mut entry: ErrorEntry) {
        // Apply context if location is not already set
        if entry.location.module.is_none() {
            entry.location.module = self.module_context.clone();
        }
        if entry.location.type_name.is_none() {
            entry.location.type_name = self.type_context.clone();
        }
        self.entries.push(entry);
    }

    /// Add a simple error message with current context
    pub fn add_error(&mut self, category: ErrorCategory, message: impl Into<String>) {
        self.add(ErrorEntry::new(category, message));
    }

    /// Check if any errors have been collected
    pub fn has_errors(&self) -> bool {
        !self.entries.is_empty()
    }

    /// Get the number of errors
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Get all error entries
    pub fn entries(&self) -> &[ErrorEntry] {
        &self.entries
    }

    /// Get errors by category
    pub fn by_category(&self, category: ErrorCategory) -> impl Iterator<Item = &ErrorEntry> {
        self.entries.iter().filter(move |e| e.category == category)
    }

    /// Merge another batch into this one
    pub fn merge(&mut self, other: BatchErrors) {
        self.entries.extend(other.entries);
    }

    /// Format errors as a summary report
    pub fn format_summary(&self) -> String {
        if self.entries.is_empty() {
            return "No errors".to_string();
        }

        // Group by category
        let mut by_category: std::collections::HashMap<ErrorCategory, Vec<&ErrorEntry>> =
            std::collections::HashMap::new();
        for entry in &self.entries {
            by_category.entry(entry.category).or_default().push(entry);
        }

        let mut lines = Vec::new();
        lines.push(format!("Found {} error(s):", self.entries.len()));
        lines.push(String::new());

        for (category, entries) in by_category {
            lines.push(format!("## {} ({} errors):", category, entries.len()));
            for entry in entries.iter().take(10) {
                lines.push(format!("  - {}", entry));
            }
            if entries.len() > 10 {
                lines.push(format!("  ... and {} more", entries.len() - 10));
            }
            lines.push(String::new());
        }

        lines.join("\n")
    }

    /// Format errors as a compact list
    pub fn format_compact(&self) -> String {
        self.entries
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<BatchErrors> for CodegenError {
    fn from(errors: BatchErrors) -> Self {
        CodegenError::Batch {
            count: errors.count(),
            summary: errors.format_summary(),
        }
    }
}

impl fmt::Display for BatchErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_summary())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_location_display() {
        let loc = ErrorLocation::new()
            .in_module("io.k8s.api.core.v1")
            .in_type("Pod")
            .in_field("metadata");
        assert_eq!(format!("{}", loc), "io.k8s.api.core.v1.Pod::metadata");

        let loc2 = ErrorLocation::new().in_module("io.k8s.api.core.v1");
        assert_eq!(format!("{}", loc2), "io.k8s.api.core.v1");
    }

    #[test]
    fn test_batch_errors_context() {
        let mut errors = BatchErrors::new();

        errors.set_module_context("test.module.v1");
        errors.set_type_context("TestType");
        errors.add_error(ErrorCategory::MissingType, "Type X not found");

        assert_eq!(errors.count(), 1);
        let entry = &errors.entries()[0];
        assert_eq!(entry.location.module.as_deref(), Some("test.module.v1"));
        assert_eq!(entry.location.type_name.as_deref(), Some("TestType"));
    }

    #[test]
    fn test_batch_errors_merge() {
        let mut errors1 = BatchErrors::new();
        errors1.add_error(ErrorCategory::MissingType, "Error 1");

        let mut errors2 = BatchErrors::new();
        errors2.add_error(ErrorCategory::InvalidModule, "Error 2");
        errors2.add_error(ErrorCategory::InvalidModule, "Error 3");

        errors1.merge(errors2);
        assert_eq!(errors1.count(), 3);
    }

    #[test]
    fn test_error_entry_helpers() {
        let entry = ErrorEntry::missing_type("ObjectMeta", "referenced in Pod.spec")
            .at(ErrorLocation::new().in_module("io.k8s.api.core.v1"))
            .with_suggestion("Import from apimachinery");

        assert!(entry.message.contains("ObjectMeta"));
        assert!(entry.suggestion.is_some());
    }

    #[test]
    fn test_by_category() {
        let mut errors = BatchErrors::new();
        errors.add_error(ErrorCategory::MissingType, "Missing 1");
        errors.add_error(ErrorCategory::InvalidModule, "Invalid 1");
        errors.add_error(ErrorCategory::MissingType, "Missing 2");

        let missing: Vec<_> = errors.by_category(ErrorCategory::MissingType).collect();
        assert_eq!(missing.len(), 2);
    }

    #[test]
    fn test_format_summary() {
        let mut errors = BatchErrors::new();
        errors.add_error(ErrorCategory::MissingType, "Type X not found");
        errors.add_error(ErrorCategory::MissingType, "Type Y not found");

        let summary = errors.format_summary();
        assert!(summary.contains("2 error(s)"));
        assert!(summary.contains("MISSING_TYPE"));
    }

    #[test]
    fn test_into_codegen_error() {
        let mut errors = BatchErrors::new();
        errors.add_error(ErrorCategory::MissingType, "Test error");

        let codegen_error: CodegenError = errors.into();
        match codegen_error {
            CodegenError::Batch { count, .. } => assert_eq!(count, 1),
            _ => panic!("Expected Batch error"),
        }
    }
}
