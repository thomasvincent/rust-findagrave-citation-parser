use thiserror::Error;

/// Custom error type for FindAGrave citation parser
#[derive(Error, Debug)]
pub enum Error {
    /// Error that occurs during HTTP requests
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Error that occurs when parsing HTML
    #[error("HTML parsing error: {0}")]
    ParseError(String),

    /// Error that occurs when accessing the database
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Error that occurs when reading or writing files
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error that occurs when serializing or deserializing JSON
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Represents a missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Represents an invalid URL or ID
    #[error("Invalid URL or memorial ID: {0}")]
    InvalidInput(String),

    /// Represents any other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for FindAGrave operations
pub type Result<T> = std::result::Result<T, Error>;

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_string() {
        let err = Error::from("test error");
        match err {
            Error::Other(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Expected Error::Other"),
        }
    }
}
