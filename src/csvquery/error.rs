use std::result;
use sqlparser::parser::ParserError;

pub type CSVQueryResult<T> = result::Result<T, CSVQueryError>;

#[derive(thiserror::Error, Debug)]
pub enum CSVQueryError {
    #[error("SQLParser Error: {0}")]
    SQLParser(#[from] ParserError),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Internal Error: {0}")]
    Internal(String),
}
