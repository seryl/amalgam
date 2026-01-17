//! Pre-flight validation for code generation
//!
//! This module validates IR before code generation to catch issues early,
//! replacing the error-marker approach with proper upfront validation.
//!
//! ## Validation Checks
//!
//! - All type references can be resolved
//! - No empty modules (or modules with only invalid types)
//! - Module names are valid
//! - Type names follow expected patterns
//!
//! ## Usage
//!
//! ```ignore
//! let validator = CodegenValidator::new(&registry);
//! let result = validator.validate(&ir);
//! if result.has_errors() {
//!     for error in result.errors() {
//!         eprintln!("Error: {}", error);
//!     }
//!     return Err(result.into());
//! }
//! ```

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use amalgam_core::ir::{Module, IR};
use amalgam_core::module_registry::ModuleRegistry;
use amalgam_core::types::Type;
use amalgam_core::Fqn;

use crate::CodegenError;

/// Severity of validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational - does not block generation
    Info,
    /// Warning - may cause issues but generation can proceed
    Warning,
    /// Error - blocks generation
    Error,
}

/// A single validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Severity of the issue
    pub severity: Severity,
    /// Module where the issue was found (if applicable)
    pub module: Option<String>,
    /// Type where the issue was found (if applicable)
    pub type_name: Option<String>,
    /// Description of the issue
    pub message: String,
    /// Suggested fix (if applicable)
    pub suggestion: Option<String>,
}

impl ValidationIssue {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            module: None,
            type_name: None,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            module: None,
            type_name: None,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Info,
            module: None,
            type_name: None,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn in_module(mut self, module: impl Into<String>) -> Self {
        self.module = Some(module.into());
        self
    }

    pub fn in_type(mut self, type_name: impl Into<String>) -> Self {
        self.type_name = Some(type_name.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

impl std::fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self.severity {
            Severity::Info => "INFO",
            Severity::Warning => "WARN",
            Severity::Error => "ERROR",
        };

        let location = match (&self.module, &self.type_name) {
            (Some(m), Some(t)) => format!(" in {}.{}", m, t),
            (Some(m), None) => format!(" in module {}", m),
            (None, Some(t)) => format!(" in type {}", t),
            (None, None) => String::new(),
        };

        write!(f, "[{}]{}: {}", severity, location, self.message)?;

        if let Some(ref suggestion) = self.suggestion {
            write!(f, " (suggestion: {})", suggestion)?;
        }

        Ok(())
    }
}

/// Result of validation
#[derive(Debug, Default)]
pub struct ValidationResult {
    issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    pub fn add(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.issues.extend(other.issues);
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }

    pub fn errors(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.severity == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues.iter().filter(|i| i.severity == Severity::Warning)
    }

    pub fn all(&self) -> &[ValidationIssue] {
        &self.issues
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count()
    }
}

impl From<ValidationResult> for CodegenError {
    fn from(result: ValidationResult) -> Self {
        let error_messages: Vec<String> = result
            .errors()
            .map(|e| e.to_string())
            .collect();

        CodegenError::Generation(format!(
            "Validation failed with {} errors:\n{}",
            error_messages.len(),
            error_messages.join("\n")
        ))
    }
}

/// Pre-flight validator for code generation
pub struct CodegenValidator {
    registry: Arc<ModuleRegistry>,
    /// Known types across all modules (for reference validation)
    known_types: HashSet<String>,
    /// Module -> types mapping
    module_types: HashMap<String, HashSet<String>>,
}

impl CodegenValidator {
    /// Create a new validator with the given module registry
    pub fn new(registry: Arc<ModuleRegistry>) -> Self {
        Self {
            registry,
            known_types: HashSet::new(),
            module_types: HashMap::new(),
        }
    }

    /// Validate an IR before code generation
    pub fn validate(&mut self, ir: &IR) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Phase 1: Build the known types index
        self.build_type_index(ir);

        // Phase 2: Validate each module
        for module in &ir.modules {
            let module_result = self.validate_module(module);
            result.merge(module_result);
        }

        // Phase 3: Cross-module validation
        let cross_result = self.validate_cross_references(ir);
        result.merge(cross_result);

        result
    }

    /// Build an index of all known types
    fn build_type_index(&mut self, ir: &IR) {
        self.known_types.clear();
        self.module_types.clear();

        for module in &ir.modules {
            let mut types = HashSet::new();
            for type_def in &module.types {
                self.known_types.insert(type_def.name.clone());
                types.insert(type_def.name.clone());
            }
            self.module_types.insert(module.name.clone(), types);
        }
    }

    /// Validate a single module
    fn validate_module(&self, module: &Module) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Check for empty module name
        if module.name.is_empty() {
            result.add(
                ValidationIssue::error("Module has empty name")
                    .with_suggestion("Ensure module name is set during parsing"),
            );
        }

        // Check for empty module (no types)
        if module.types.is_empty() {
            result.add(
                ValidationIssue::warning("Module has no types defined")
                    .in_module(&module.name)
                    .with_suggestion("Consider removing empty modules from output"),
            );
        }

        // Validate each type in the module
        for type_def in &module.types {
            let type_result = self.validate_type(&module.name, &type_def.name, &type_def.ty);
            result.merge(type_result);
        }

        result
    }

    /// Validate a type definition
    fn validate_type(&self, module: &str, type_name: &str, ty: &Type) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Check for empty type name
        if type_name.is_empty() {
            result.add(
                ValidationIssue::error("Type has empty name")
                    .in_module(module)
                    .with_suggestion("Check parser for type name extraction issues"),
            );
            return result;
        }

        // Check type name starts with uppercase (convention)
        if !type_name.chars().next().map_or(false, |c| c.is_ascii_uppercase()) {
            result.add(
                ValidationIssue::warning("Type name should start with uppercase letter")
                    .in_module(module)
                    .in_type(type_name)
                    .with_suggestion(format!(
                        "Rename to {}",
                        Self::to_pascal_case(type_name)
                    )),
            );
        }

        // Validate type references within this type
        self.validate_type_references(module, type_name, ty, &mut result);

        result
    }

    /// Validate type references within a type
    fn validate_type_references(
        &self,
        module: &str,
        type_name: &str,
        ty: &Type,
        result: &mut ValidationResult,
    ) {
        match ty {
            Type::Reference { module: ref_module, name: ref_name } => {
                // Check if the referenced type exists
                let full_ref = match ref_module {
                    Some(m) if !m.is_empty() => format!("{}.{}", m, ref_name),
                    _ => ref_name.clone(),
                };

                // First check: is it in our known types?
                if !self.known_types.contains(ref_name) {
                    // Check registry
                    if self.registry.find_module_for_type(ref_name).is_none() {
                        // Try parsing as FQN
                        if let Ok(fqn) = Fqn::parse(&full_ref) {
                            if !self.known_types.contains(fqn.type_name()) {
                                result.add(
                                    ValidationIssue::error(format!(
                                        "Reference to unknown type: {}",
                                        full_ref
                                    ))
                                    .in_module(module)
                                    .in_type(type_name)
                                    .with_suggestion("Check if the referenced type exists in the schema"),
                                );
                            }
                        }
                    }
                }
            }
            Type::Array(inner) => {
                self.validate_type_references(module, type_name, inner, result);
            }
            Type::Map { value, .. } => {
                self.validate_type_references(module, type_name, value, result);
            }
            Type::Optional(inner) => {
                self.validate_type_references(module, type_name, inner, result);
            }
            Type::Record { fields, .. } => {
                for (_field_name, field) in fields {
                    self.validate_type_references(module, type_name, &field.ty, result);
                }
            }
            Type::Union { types, .. } => {
                for variant in types {
                    self.validate_type_references(module, type_name, variant, result);
                }
            }
            Type::TaggedUnion { variants, .. } => {
                for (_, variant_type) in variants {
                    self.validate_type_references(module, type_name, variant_type, result);
                }
            }
            Type::Constrained { base_type, .. } => {
                self.validate_type_references(module, type_name, base_type, result);
            }
            Type::Contract { base, .. } => {
                self.validate_type_references(module, type_name, base, result);
            }
            // Primitives don't have references
            Type::String
            | Type::Number
            | Type::Integer
            | Type::Bool
            | Type::Null
            | Type::Any => {}
        }
    }

    /// Validate cross-module references
    fn validate_cross_references(&self, _ir: &IR) -> ValidationResult {
        let result = ValidationResult::new();

        // Additional cross-module validations can go here
        // For now, the per-type reference validation handles most cases

        result
    }

    /// Convert to PascalCase
    fn to_pascal_case(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }
        let mut chars: Vec<char> = s.chars().collect();
        chars[0] = chars[0].to_ascii_uppercase();
        chars.into_iter().collect()
    }
}

/// Validate IR and return errors if any
pub fn validate_ir(ir: &IR, registry: Arc<ModuleRegistry>) -> Result<(), CodegenError> {
    let mut validator = CodegenValidator::new(registry);
    let result = validator.validate(ir);

    if result.has_errors() {
        Err(result.into())
    } else {
        // Log warnings but don't fail
        for warning in result.warnings() {
            tracing::warn!("{}", warning);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amalgam_core::ir::{Module, TypeDefinition};

    fn test_registry() -> Arc<ModuleRegistry> {
        Arc::new(ModuleRegistry::new())
    }

    #[test]
    fn test_empty_ir_is_valid() {
        let ir = IR::new();
        let mut validator = CodegenValidator::new(test_registry());
        let result = validator.validate(&ir);
        assert!(!result.has_errors());
    }

    #[test]
    fn test_empty_module_warning() {
        let mut ir = IR::new();
        ir.modules.push(Module {
            name: "test.module.v1".to_string(),
            types: vec![],
            imports: vec![],
            constants: vec![],
            metadata: Default::default(),
        });

        let mut validator = CodegenValidator::new(test_registry());
        let result = validator.validate(&ir);

        assert!(!result.has_errors());
        assert!(result.has_warnings());
        assert!(result.warnings().any(|w| w.message.contains("no types")));
    }

    #[test]
    fn test_empty_module_name_error() {
        let mut ir = IR::new();
        ir.modules.push(Module {
            name: String::new(),
            types: vec![TypeDefinition {
                name: "Test".to_string(),
                ty: Type::String,
                documentation: None,
                annotations: Default::default(),
            }],
            imports: vec![],
            constants: vec![],
            metadata: Default::default(),
        });

        let mut validator = CodegenValidator::new(test_registry());
        let result = validator.validate(&ir);

        assert!(result.has_errors());
        assert!(result.errors().any(|e| e.message.contains("empty name")));
    }

    #[test]
    fn test_empty_type_name_error() {
        let mut ir = IR::new();
        ir.modules.push(Module {
            name: "test.module.v1".to_string(),
            types: vec![TypeDefinition {
                name: String::new(),
                ty: Type::String,
                documentation: None,
                annotations: Default::default(),
            }],
            imports: vec![],
            constants: vec![],
            metadata: Default::default(),
        });

        let mut validator = CodegenValidator::new(test_registry());
        let result = validator.validate(&ir);

        assert!(result.has_errors());
        assert!(result.errors().any(|e| e.message.contains("empty name")));
    }

    #[test]
    fn test_lowercase_type_name_warning() {
        let mut ir = IR::new();
        ir.modules.push(Module {
            name: "test.module.v1".to_string(),
            types: vec![TypeDefinition {
                name: "myType".to_string(),
                ty: Type::String,
                documentation: None,
                annotations: Default::default(),
            }],
            imports: vec![],
            constants: vec![],
            metadata: Default::default(),
        });

        let mut validator = CodegenValidator::new(test_registry());
        let result = validator.validate(&ir);

        assert!(!result.has_errors());
        assert!(result.has_warnings());
        assert!(result.warnings().any(|w| w.message.contains("uppercase")));
    }

    #[test]
    fn test_validation_issue_display() {
        let issue = ValidationIssue::error("Test error")
            .in_module("test.module")
            .in_type("TestType")
            .with_suggestion("Fix it");

        let display = format!("{}", issue);
        assert!(display.contains("ERROR"));
        assert!(display.contains("test.module"));
        assert!(display.contains("TestType"));
        assert!(display.contains("Test error"));
        assert!(display.contains("Fix it"));
    }
}
