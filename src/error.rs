use thiserror::Error;

pub type Result<T> = std::result::Result<T, ValidationError>;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Schema validation error: {0}")]
    SchemaValidation(String),

    #[error("Schema loading error: {0}")]
    SchemaLoad(#[from] std::io::Error),

    #[error("Invalid IR structure: {0}")]
    InvalidStructure(String),
}
