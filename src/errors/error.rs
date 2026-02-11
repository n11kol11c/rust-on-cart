#[derive(Debug)]
pub enum CartError {
    IoError(String),
    DbError(String),
    NetworkError(String),
    ResourceError(String),
    Exception(String),
}

impl std::fmt::Display for CartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CartError::IoError(msg) => write!(f, "IO Error: {}", msg),
            CartError::DbError(msg) => write!(f, "Database Error: {}", msg),
            CartError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            CartError::ResourceError(msg) => write!(f, "Resource Error: {}", msg),
            CartError::Exception(msg) => write!(f, "Error: {}", msg),
            CartError::EnumError(msg) => write!(f, "Enum Error: {}", msg),
        }
    }
}


impl std::error::Error for CartError {}
