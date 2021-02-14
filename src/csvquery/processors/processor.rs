use async_trait::async_trait;
use std::sync::Arc;
use crate::csvquery::error::CSVQueryResult;

pub type ProcessorRef = Arc<dyn IProcessor>;

#[async_trait]
pub trait IProcessor {
    fn name(&self) -> &'static str;

    fn connect_to(&mut self, input: ProcessorRef);

    async fn execute(&self) -> CSVQueryResult<()>;
}
