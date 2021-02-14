use crate::csvquery::data_schema::DataSchemaRef;
use std::sync::Arc;

pub type DataSourceRef = Arc<dyn DataSource>;

pub trait DataSource {
    fn schema(&self) -> DataSchemaRef;
}
