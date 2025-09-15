//! Code generators for various target languages

pub mod error;
pub mod go;
pub mod import_pipeline_debug;
pub mod nickel;
pub mod nickel_manifest;
pub mod nickel_package;
pub mod nickel_rich;
pub mod package_mode;
pub mod resolver;

// Test debug utilities are public for integration tests
pub mod test_debug;

use amalgam_core::IR;

pub use error::CodegenError;

/// Common trait for all code generators
pub trait Codegen {
    fn generate(&mut self, ir: &IR) -> Result<String, CodegenError>;
}
