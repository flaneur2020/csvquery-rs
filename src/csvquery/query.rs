use crate::csvquery::data_block::DataBlockRef;
use crate::csvquery::error::CSVQueryError;

pub struct CSVQueryExecutor<'a> {
    sql: &'a str,
}

impl<'a> CSVQueryExecutor<'a> {
    pub fn new(sql: &'a str) -> Self {
        Self { sql: sql }
    }

    pub fn execute(&self) -> Result<DataBlockRef, CSVQueryError> {
        Err(CSVQueryError::Internal(format!("failed")))
    }
}
