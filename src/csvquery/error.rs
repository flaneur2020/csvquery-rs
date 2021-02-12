use sqlparser::parser::ParserError;

#[derive(thiserror::Error, Debug)]
pub enum CSVQueryError {
    #[error("SQLParser Error: {0}")]
    SQLParser(#[from] ParserError),

    #[error("Internal Error: {0}")]
    Internal(String),
}
