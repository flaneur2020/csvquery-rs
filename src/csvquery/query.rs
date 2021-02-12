use crate::csvquery::error::CSVQueryError;
use crate::csvquery::table_data::DataSet;

pub struct CSVQueryExecutor<'a> {
    sql: &'a str,
}

impl<'a> CSVQueryExecutor<'a> {
    pub fn new(sql: &'a str) -> Self {
        Self {
            sql: sql,
        }
    }

    pub fn execute(&self) -> Result<DataSet, CSVQueryError> {
        Ok(DataSet::empty())
    }
}