use std::io;
use thiserror::{self, Error};


#[derive(Error, Debug)]
pub enum StructuredError {
    #[error("Structured :: ParseError: {0}")]
    ParseError(String),
    #[error("Structured :: WriterError: {0}")]
    WriterError(String),
    #[error("Structured :: IoError: {0}")]
    IoError(String)
}

impl From<serde_json::Error> for StructuredError {
    fn from(se: serde_json::Error) -> Self {
        StructuredError::ParseError(se.to_string())
    }
}

impl From<tracing::metadata::ParseLevelError> for StructuredError {
    fn from(tep: tracing::metadata::ParseLevelError) -> Self {
        StructuredError::WriterError(tep.to_string())
    }
}

impl From<io::Error> for StructuredError {
    fn from(ioe: io::Error) -> Self {
        StructuredError::IoError(ioe.to_string())
    }
}

pub type Result<T> = std::result::Result<T, StructuredError>;
