use crate::csvquery::data_streams::{ChannelStream, SendableDataBlockStream, CsvStream};
use crate::csvquery::data_types::{DataBlock, DataSchemaRef};
use crate::csvquery::plans::{ScanPlan};
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::processors::{IProcessor, ProcessorRef};
use async_trait::async_trait;
use futures::StreamExt;

pub struct ScanProcessor {
    schema: DataSchemaRef,
    stream: SendableDataBlockStream,
}

impl ScanProcessor {
    pub fn new(schema: DataSchemaRef, stream: SendableDataBlockStream) -> Self {
        Self {
            schema,
            stream,
        }
    }
}

#[async_trait]
impl IProcessor for ScanProcessor {
    fn name(&self) -> &'static str {
        "ScanProcessor"
    }

    fn connect_to(&mut self, input: ProcessorRef) -> CSVQueryResult<()> {
        Err(CSVQueryError::Internal(
            "Can not connect ScanProcessor to another processor".to_string(),
        ))
    }

    async fn execute(&self) -> CSVQueryResult<SendableDataBlockStream> {
        Err(CSVQueryError::Internal("not implemented".to_string()))
    }
}
