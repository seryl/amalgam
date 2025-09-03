//! Core intermediate representation and type system for amalgam

pub mod debug;
pub mod dependency_analyzer;
pub mod error;
pub mod fingerprint;
pub mod import_calculator;
pub mod ir;
pub mod types;

pub use debug::{CompilationDebugInfo, DebugConfig};
pub use error::CoreError;
pub use import_calculator::ImportPathCalculator;
pub use ir::IR;
pub use types::{Type, TypeSystem};
