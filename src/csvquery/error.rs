use arrow::error::ArrowError;
use sqlparser::parser::ParserError;
use std::io;
use std::result;

pub type CQResult<T> = result::Result<T, CQError>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CQError {
    #[error("SQLParser Error: {0}")]
    SQLParser(#[from] ParserError),

    #[error("Field Not Found: {0}")]
    FieldNotFound(String),

    #[error("Not Implemented: {0}")]
    NotImplemented(String),

    #[error("Internal Error: {0}")]
    Internal(String),
}

impl From<io::Error> for CQError {
    fn from(err: std::io::Error) -> Self {
        CQError::Internal(err.to_string())
    }
}

impl From<ArrowError> for CQError {
    fn from(err: ArrowError) -> Self {
        CQError::Internal(err.to_string())
    }
}
