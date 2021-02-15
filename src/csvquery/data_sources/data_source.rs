use crate::csvquery::data_streams::SendableDataBlockStream;
use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::error::CQResult;
use std::sync::Arc;

pub type DataSourceRef = Arc<dyn DataSource + Send + Sync>;

pub trait DataSource {
    fn name(&self) -> String;

    fn schema(&self) -> CQResult<DataSchemaRef>;

    fn stream(&self) -> CQResult<SendableDataBlockStream>;
}
