use std::io;
use sqlparser::parser::ParserError;
use std::result;
use arrow::error::ArrowError;

pub type CSVQueryResult<T> = result::Result<T, CSVQueryError>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CSVQueryError {
    #[error("SQLParser Error: {0}")]
    SQLParser(#[from] ParserError),

    #[error("Field Not Found: {0}")]
    FieldNotFound(String),

    #[error("Not Implemented: {0}")]
    NotImplemented(String),

    #[error("Internal Error: {0}")]
    Internal(String),
}

impl From<io::Error> for CSVQueryError {
    fn from(err: std::io::Error) -> Self {
        CSVQueryError::Internal(err.to_string())
    }
}

impl From<ArrowError> for CSVQueryError {
    fn from(err: ArrowError) -> Self {
        CSVQueryError::Internal(err.to_string())
    }
}