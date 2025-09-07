use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Type conversion error: {0}")]
    TypeConversion(String),

    #[error("Invalid type definition: {0}")]
    InvalidType(String),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),
    
    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
