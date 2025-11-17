//! Error types for verification

use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, VerificationError>;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Import binding mismatch in {file}: binding '{binding}' doesn't match usage '{usage}'")]
    BindingMismatch {
        file: PathBuf,
        binding: String,
        usage: String,
    },

    #[error("Dangling reference in {file}: type '{type_name}' used but not imported")]
    DanglingReference {
        file: PathBuf,
        type_name: String,
    },

    #[error("Nickel type check failed for {file}: {stderr}")]
    TypeCheckFailed {
        file: PathBuf,
        stderr: String,
    },

    #[error("YAML round-trip failed for {file}: outputs don't match")]
    RoundTripFailed {
        file: PathBuf,
        diff: String,
    },

    #[error("Schema validation failed for {file}: {message}")]
    SchemaValidationFailed {
        file: PathBuf,
        message: String,
    },

    #[error("Nickel executable not found. Please install nickel: https://nickel-lang.org/")]
    NickelNotFound,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Process execution failed: {0}")]
    ProcessFailed(String),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("{0}")]
    Other(String),
}
