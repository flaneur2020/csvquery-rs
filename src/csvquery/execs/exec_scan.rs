use crate::csvquery::streams::{ChannelStream, CsvStream, SendableDataBlockStream};
use crate::csvquery::data_types::{DataBlock, DataSchemaRef};
use crate::csvquery::data_sources::{DataSource, DataSourceRef};
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::execs::{Execution, ExecutionRef};
use async_trait::async_trait;
use futures::StreamExt;

pub struct ScanExecution {
    schema: DataSchemaRef,
    datasource: DataSourceRef,
}

impl ScanExecution {
    pub fn new(schema: DataSchemaRef, datasource: DataSourceRef) -> Self {
        Self { schema, datasource }
    }
}

#[async_trait]
impl Execution for ScanExecution {
    fn name(&self) -> &'static str {
        "ScanExecution"
    }

    fn connect_to(&mut self, _: ExecutionRef) -> CQResult<()> {
        Err(CQError::Internal(
            "can not connect source execution".to_string(),
        ))
    }

    async fn execute(&self) -> CQResult<SendableDataBlockStream> {
        Err(CQError::Internal("not implemented".to_string()))
    }
}
