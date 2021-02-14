use crate::csvquery::data_streams::DataBlockStream;
use crate::csvquery::error::CSVQueryResult;
use async_trait::async_trait;
use std::sync::Arc;

pub type ProcessorRef = Arc<dyn IProcessor>;

#[async_trait]
pub trait IProcessor: Sync + Send {
    fn name(&self) -> &'static str;

    fn connect_to(&mut self, input: ProcessorRef) -> CSVQueryResult<()>;

    async fn execute(&self) -> CSVQueryResult<DataBlockStream>;
}
