//! Core intermediate representation and type system for amalgam

pub mod dependency_analyzer;
pub mod error;
pub mod fingerprint;
pub mod ir;
pub mod types;

pub use error::CoreError;
pub use ir::IR;
pub use types::{Type, TypeSystem};
