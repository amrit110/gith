//! Error types for the gith application

use std::fmt;

/// Custom error type for gith operations
#[derive(Debug)]
pub enum GithError {
    /// Git operation failed
    GitError(String),
    /// IO operation failed
    IoError(std::io::Error),
    /// JSON parsing/serialization failed
    JsonError(serde_json::Error),
    /// Repository not found or not a git repository
    NotARepository,
    /// Invalid configuration
    ConfigError(String),
    /// Generic error with message
    Other(String),
}

impl fmt::Display for GithError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GithError::GitError(msg) => write!(f, "Git error: {}", msg),
            GithError::IoError(err) => write!(f, "IO error: {}", err),
            GithError::JsonError(err) => write!(f, "JSON error: {}", err),
            GithError::NotARepository => write!(f, "Not a git repository"),
            GithError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            GithError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for GithError {}

impl From<std::io::Error> for GithError {
    fn from(err: std::io::Error) -> Self {
        GithError::IoError(err)
    }
}

impl From<serde_json::Error> for GithError {
    fn from(err: serde_json::Error) -> Self {
        GithError::JsonError(err)
    }
}

impl From<git2::Error> for GithError {
    fn from(err: git2::Error) -> Self {
        GithError::GitError(err.message().to_string())
    }
}

/// Result type alias for gith operations
pub type Result<T> = std::result::Result<T, GithError>;
