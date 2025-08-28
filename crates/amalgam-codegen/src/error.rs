use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Code generation error: {0}")]
    Generation(String),

    #[error("Unsupported type: {0}")]
    UnsupportedType(String),

    #[error("Invalid IR: {0}")]
    InvalidIR(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Fmt(#[from] std::fmt::Error),
}
