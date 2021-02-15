use crate::csvquery::data_streams::SendableDataBlockStream;
use crate::csvquery::error::CSVQueryResult;
use async_trait::async_trait;
use std::sync::Arc;

pub type ExecutionRef = Arc<dyn Execution>;

#[async_trait]
pub trait Execution: Sync + Send {
    fn name(&self) -> &'static str;

    fn connect_to(&mut self, input: ExecutionRef) -> CSVQueryResult<()>;

    async fn execute(&self) -> CSVQueryResult<SendableDataBlockStream>;
}
