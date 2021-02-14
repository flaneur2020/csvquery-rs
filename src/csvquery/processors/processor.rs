use std::sync::Arc;
use crate::csvquery::error::CSVQueryResult;

pub type ProcessorRef = Arc<dyn IProcessor>;

pub trait IProcessor {
    fn name(&self) -> &'static str;

    fn connect_to(&mut self, input: ProcessorRef);

    fn execute(&self) -> CSVQueryResult<()>;
}
