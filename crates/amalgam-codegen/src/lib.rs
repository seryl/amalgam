//! Code generators for various target languages

pub mod nickel;
pub mod go;
pub mod error;
pub mod resolver;

use amalgam_core::IR;

pub use error::CodegenError;

/// Common trait for all code generators
pub trait Codegen {
    fn generate(&mut self, ir: &IR) -> Result<String, CodegenError>;
}