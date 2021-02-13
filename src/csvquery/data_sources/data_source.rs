use crate::csvquery::data_block::DataBlockRef;
use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::error::CSVQueryError;
use std::sync::Arc;

pub type DataSourceRef = Arc<dyn DataSource>;

pub trait DataSource {
    fn schema(&self) -> DataSchemaRef;
}
