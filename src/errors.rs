use thiserror::Error;

#[derive(Error, Debug)]
pub enum CartError {
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Callback failed: {0}")]
    CallbackError(String),
}