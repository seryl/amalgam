//! Stub implementations when the `nickel` feature is disabled.
//!
//! These types provide a compile-time API surface but return errors
//! indicating that Nickel support is not available.

use crate::errors::{ValidationError, ValidationErrors};
use serde::Serialize;
use std::path::Path;

/// Stub validator when Nickel feature is disabled.
#[derive(Debug)]
pub struct NickelValidator {
    _private: (),
}

/// Stub configuration.
#[derive(Debug, Clone, Default)]
pub struct ValidatorConfig {
    _private: (),
}

impl NickelValidator {
    /// Stub implementation that returns an error.
    pub fn from_package(_package_root: impl AsRef<Path>) -> Result<Self, ValidationErrors> {
        Err(ValidationErrors::from_error(ValidationError::new(
            "",
            "Nickel validation is not available: compile with the 'nickel' feature enabled",
        )))
    }

    /// Stub implementation.
    pub fn with_config(self, _config: ValidatorConfig) -> Self {
        self
    }

    /// Stub implementation that returns an error.
    pub fn validate<T: Serialize>(
        &self,
        _value: &T,
        _contract_name: &str,
    ) -> Result<(), ValidationErrors> {
        Err(ValidationErrors::from_error(ValidationError::new(
            "",
            "Nickel validation is not available: compile with the 'nickel' feature enabled",
        )))
    }

    /// Stub implementation that returns an error.
    pub fn validate_json(
        &self,
        _value: &serde_json::Value,
        _contract_name: &str,
    ) -> Result<(), ValidationErrors> {
        Err(ValidationErrors::from_error(ValidationError::new(
            "",
            "Nickel validation is not available: compile with the 'nickel' feature enabled",
        )))
    }
}

/// Stub trait for Nickel validation.
pub trait ValidateWithNickel: Serialize {
    /// Stub implementation that returns an error.
    fn validate_with_nickel(
        &self,
        _validator: &NickelValidator,
        _contract_name: &str,
    ) -> Result<(), ValidationErrors> {
        Err(ValidationErrors::from_error(ValidationError::new(
            "",
            "Nickel validation is not available: compile with the 'nickel' feature enabled",
        )))
    }
}

impl<T: Serialize> ValidateWithNickel for T {}
