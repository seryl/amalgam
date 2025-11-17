//! Amalgam Verification Suite
//!
//! Comprehensive validation system for Amalgam-generated Nickel code.
//!
//! This crate provides multi-level validation:
//! 1. Import binding resolution (no dangling references)
//! 2. Nickel type checking (static validation)
//! 3. YAML round-trip testing (semantic equivalence)
//! 4. Schema validation (kubectl/kubeconform)
//!
//! # Example
//!
//! ```no_run
//! use amalgam_verification::Validator;
//!
//! let validator = Validator::new("tests/fixtures/generated");
//! let report = validator.validate_all();
//!
//! assert!(report.is_success());
//! ```

pub mod binding_resolver;
pub mod nickel_typechecker;
pub mod reporter;
pub mod schema_validator;
pub mod yaml_roundtrip;

mod error;
mod validator;

pub use error::{VerificationError, Result};
pub use validator::Validator;
pub use reporter::ValidationReport;

/// Re-exports for convenience
pub mod prelude {
    pub use crate::binding_resolver::BindingResolver;
    pub use crate::nickel_typechecker::NickelTypeChecker;
    pub use crate::schema_validator::SchemaValidator;
    pub use crate::yaml_roundtrip::YamlRoundTrip;
    pub use crate::reporter::ValidationReport;
    pub use crate::validator::Validator;
    pub use crate::{Result, VerificationError};
}
