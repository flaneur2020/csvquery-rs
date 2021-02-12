use crate::csvquery::error::CSVQueryError;
use crate::csvquery::data_row::DataRow;

pub struct CSVQueryExecutor<'a> {
    sql: &'a str,
}

impl<'a> CSVQueryExecutor<'a> {
    pub fn new(sql: &'a str) -> Self {
        Self {
            sql: sql,
        }
    }

    pub fn execute(&self) -> Result<Vec<DataRow>, CSVQueryError> {
        let v: Vec<DataRow> = vec![];
        Ok(v)
    }
}