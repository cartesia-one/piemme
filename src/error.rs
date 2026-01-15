//! Application error types

use thiserror::Error;

/// Application-specific errors
#[derive(Error, Debug)]
pub enum PiemmeError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Prompt not found: {0}")]
    PromptNotFound(String),

    #[error("Invalid prompt name: {0}")]
    InvalidPromptName(String),

    #[error("Duplicate prompt name: {0}")]
    DuplicateName(String),

    #[error("Circular reference detected: {0}")]
    CircularReference(String),

    #[error("Maximum reference depth exceeded")]
    MaxDepthExceeded,

    #[error("Command execution failed: {0}")]
    CommandExecution(String),

    #[error("Clipboard error: {0}")]
    Clipboard(String),

    #[error("YAML parsing error: {0}")]
    YamlParse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for Piemme errors
pub type PiemmeResult<T> = Result<T, PiemmeError>;
