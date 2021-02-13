use std::sync::Arc;
use crate::csvquery::data_block::DataBlockRef;
use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::error::CSVQueryError;

pub type DataSourceRef = Arc<Box<dyn IDataSource>>;

pub trait IDataSource {
    fn data_block(&self, n: i32) -> Result<DataBlockRef, CSVQueryError>;

    fn schema(&self) -> DataSchemaRef;
}