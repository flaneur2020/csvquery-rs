use crate::csvquery::streams::{ChannelStream, CsvStream, CsvReadOptions, SendableDataBlockStream};
use crate::csvquery::data_types::{DataBlock, DataSchemaRef};
use crate::csvquery::data_sources::{DataSource, DataSourceRef};
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::execs::{Execution, ExecutionRef};
use async_trait::async_trait;
use futures::StreamExt;

pub struct CsvExecution {
    filename: String,
    schema: DataSchemaRef,
    projection: Option<Vec<usize>>,
    read_options: CsvReadOptions,
}

impl CsvExecution {
    pub fn new(
        filename: &str,
        schema: DataSchemaRef,
        projection: Option<Vec<usize>>,
        read_options: &CsvReadOptions,
    ) -> Self {
        Self {
            filename: filename.to_string(),
            schema: schema,
            projection: projection,
            read_options: read_options.clone()
        }
    }
}

#[async_trait]
impl Execution for CsvExecution {
    fn name(&self) -> &'static str {
        "ScanCsvExecution"
    }

    fn inputs(&self) -> Vec<ExecutionRef> {
        vec![]
    }

    fn connect_to(&mut self, _: ExecutionRef) -> CQResult<()> {
        Err(CQError::Internal(
            "can not connect source execution".to_string(),
        ))
    }

    async fn execute(&self) -> CQResult<SendableDataBlockStream> {
        CsvStream::try_new(
            &self.filename,
            self.schema.clone(),
            self.projection.clone(),
            self.read_options.clone(),
        )?;
        Err(CQError::Internal("not implemented".to_string()))
    }
}
