//! Schema parsers for various formats

pub mod crd;
pub mod dependency_graph;
pub mod error;
pub mod go;
pub mod imports;
pub mod openapi;
pub mod package;
pub mod package_walker;
pub mod parsing_trace;
pub mod swagger;
pub mod validation_extractor;
pub mod walkers;

// Native-only modules (require tokio, reqwest, etc.)
#[cfg(feature = "native")]
pub mod fetch;
#[cfg(feature = "native")]
pub mod go_ast;
#[cfg(feature = "native")]
pub mod incremental;
#[cfg(feature = "native")]
pub mod k8s_authoritative;
#[cfg(feature = "native")]
pub mod k8s_types;

use amalgam_core::IR;

pub use error::ParserError;

/// Common trait for all parsers
pub trait Parser {
    type Input;

    fn parse(&self, input: Self::Input) -> Result<IR, ParserError>;
}
