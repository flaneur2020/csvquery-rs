use crate::csvquery::data_streams::{ChannelStream, SendableDataBlockStream, CsvStream};
use crate::csvquery::data_types::{DataBlock, DataSchemaRef};
use crate::csvquery::plans::{ScanPlan};
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::execs::{Execution, ExecutionRef};
use async_trait::async_trait;
use futures::StreamExt;

pub struct ScanExecution {
    schema: DataSchemaRef,
    stream: SendableDataBlockStream,
}

impl ScanExecution {
    pub fn new(schema: DataSchemaRef, stream: SendableDataBlockStream) -> Self {
        Self {
            schema,
            stream,
        }
    }
}

#[async_trait]
impl Execution for ScanExecution {
    fn name(&self) -> &'static str {
        "ScanProcessor"
    }

    fn connect_to(&mut self, input: ExecutionRef) -> CSVQueryResult<()> {
        Err(CSVQueryError::Internal("can not connect source execution".to_string()))
    }

    async fn execute(&self) -> CSVQueryResult<SendableDataBlockStream> {
        Err(CSVQueryError::Internal("not implemented".to_string()))
    }
}
