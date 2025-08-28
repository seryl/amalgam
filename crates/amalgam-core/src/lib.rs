//! Core intermediate representation and type system for amalgam

pub mod ir;
pub mod types;
pub mod error;

pub use error::CoreError;
pub use ir::IR;
pub use types::{Type, TypeSystem};