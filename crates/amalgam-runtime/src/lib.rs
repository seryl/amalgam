//! Amalgam Runtime Library
//!
//! This crate provides runtime support for amalgam-generated types, including:
//!
//! - **Merge trait**: Deep merging of configuration objects (like Nickel's `&` operator)
//! - **Validate trait**: Validation support with Nickel contract integration
//! - **Error types**: Structured validation errors with JSON path support
//!
//! # Example
//!
//! ```rust,ignore
//! use amalgam_runtime::{Merge, Validate};
//!
//! // Merge two configurations
//! let base = Deployment::new().with_name("app");
//! let overlay = Deployment::new().with_replicas(3);
//! let merged = base.merge(overlay);
//!
//! // Validate with schema constraints
//! merged.validate()?;
//! ```
//!
//! # Nickel Integration (optional)
//!
//! When the `nickel` feature is enabled, you can validate against Nickel contracts:
//!
//! ```rust,ignore
//! use amalgam_runtime::NickelValidator;
//!
//! let validator = NickelValidator::from_package("./pkgs")?;
//! deployment.validate_with_nickel(&validator, "k8s.apps.v1.Deployment")?;
//! ```

mod errors;
mod merge;
mod validate;

// Re-export public types
pub use errors::{ValidationError, ValidationErrors};
pub use merge::{Merge, OptionMergeExt};
pub use validate::{Validate, ValidateOption, ValidateVec, ValidationContext, ValidationState};

// Validation helper functions
pub use validate::{
    validate_enum, validate_max_length, validate_maximum, validate_min_length, validate_minimum,
    validate_pattern,
};

// Nickel integration (feature-gated)
#[cfg(feature = "nickel")]
mod nickel;
#[cfg(feature = "nickel")]
pub use nickel::{NickelValidator, ValidateWithNickel, ValidatorConfig};

// Stub types when nickel feature is disabled
#[cfg(not(feature = "nickel"))]
mod nickel_stub;
#[cfg(not(feature = "nickel"))]
pub use nickel_stub::{NickelValidator, ValidateWithNickel, ValidatorConfig};
