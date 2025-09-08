//! Core intermediate representation and type system for amalgam

pub mod compilation_unit;
pub mod debug;
pub mod dependency_analyzer;
pub mod discovery;
pub mod error;
pub mod fingerprint;
pub mod import_calculator;
pub mod ir;
pub mod module_registry;
pub mod naming;
pub mod special_cases;
pub mod types;

pub use compilation_unit::{CompilationUnit, ModuleAnalysis, TypeLocation};
pub use debug::{CompilationDebugInfo, DebugConfig};
pub use error::CoreError;
pub use import_calculator::ImportPathCalculator;
pub use ir::IR;
pub use module_registry::ModuleRegistry;
pub use types::{Type, TypeSystem};
