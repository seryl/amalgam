//! Schema parsers for various formats

pub mod crd;
pub mod dependency_graph;
pub mod error;
pub mod incremental;
pub mod fetch;
pub mod go;
pub mod go_ast;
pub mod imports;
pub mod k8s_authoritative;
pub mod k8s_imports;
pub mod k8s_types;
pub mod openapi;
pub mod package;

use amalgam_core::IR;

pub use error::ParserError;

/// Common trait for all parsers
pub trait Parser {
    type Input;

    fn parse(&self, input: Self::Input) -> Result<IR, ParserError>;
}
