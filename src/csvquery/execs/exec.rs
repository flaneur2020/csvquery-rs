use crate::csvquery::error::CQResult;
use crate::csvquery::streams::SendableRecordBatchStream;
use async_trait::async_trait;
use std::sync::Arc;

pub type ExecutionRef = Arc<dyn Execution>;

#[async_trait]
pub trait Execution: Sync + Send {
    fn name(&self) -> &'static str;

    fn connect_to(&mut self, input: ExecutionRef) -> CQResult<()>;

    fn inputs(&self) -> Vec<ExecutionRef>;

    async fn execute(&self) -> CQResult<SendableRecordBatchStream>;
}
