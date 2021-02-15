use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::data_streams::DataBlockStream;
use std::sync::Arc;

pub type DataSourceRef = Arc<dyn DataSource>;

pub trait DataSource {
    fn schema(&self) -> DataSchemaRef;

    fn stream(&self) -> DataBlockStream;
}