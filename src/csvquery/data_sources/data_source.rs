use crate::csvquery::streams::SendableDataBlockStream;
use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::error::CQResult;
use crate::csvquery::execs::ExecutionRef;
use std::sync::Arc;

pub type DataSourceRef = Arc<dyn DataSource + Send + Sync>;

pub trait DataSource {
    fn name(&self) -> String;

    fn schema(&self) -> CQResult<DataSchemaRef>;

    fn scan(&self) -> CQResult<Vec<ExecutionRef>>;
}
