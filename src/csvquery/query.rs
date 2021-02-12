use crate::csvquery::error::CSVQueryError;
use crate::csvquery::table_data::DataBlockRef;

pub struct CSVQueryExecutor<'a> {
    sql: &'a str,
}

impl<'a> CSVQueryExecutor<'a> {
    pub fn new(sql: &'a str) -> Self {
        Self {
            sql: sql,
        }
    }

    pub fn execute(&self) -> Result<DataBlockRef, CSVQueryError> {
        Err(CSVQueryError::Internal(format!("failed")))
    }
}