//! Schema parsers for various formats

pub mod crd;
pub mod go;
pub mod openapi;
pub mod error;
pub mod fetch;
pub mod package;
pub mod imports;
pub mod dependency_graph;
pub mod k8s_types;
pub mod go_ast;
pub mod k8s_authoritative;

use amalgam_core::IR;

pub use error::ParserError;

/// Common trait for all parsers
pub trait Parser {
    type Input;
    
    fn parse(&self, input: Self::Input) -> Result<IR, ParserError>;
}