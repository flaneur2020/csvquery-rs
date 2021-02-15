use crate::csvquery::data_streams::SendableDataBlockStream;
use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::error::CSVQueryResult;
use std::sync::Arc;

pub type DataSourceRef = Arc<dyn DataSource>;

pub trait DataSource {
    fn name(&self) -> String;

    fn schema(&self) -> CSVQueryResult<DataSchemaRef>;

    fn streams(self) -> CSVQueryResult<Vec<SendableDataBlockStream>>;
}
