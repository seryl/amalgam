use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
}