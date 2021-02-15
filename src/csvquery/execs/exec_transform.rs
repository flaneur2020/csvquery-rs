use crate::csvquery::data_streams::{SendableDataBlockStream, TransformFunc, TransformedStream};
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::execs::{Execution, ExecutionRef};
use async_trait::async_trait;

pub struct TransformExecution {
    input: Option<ExecutionRef>,
    transform_name: &'static str,
    transform_func: TransformFunc,
}

impl TransformExecution {
    pub fn new(transform_name: &'static str, transform_func: TransformFunc) -> Self {
        Self {
            input: None,
            transform_name,
            transform_func,
        }
    }
}

#[async_trait]
impl Execution for TransformExecution {
    fn name(&self) -> &'static str {
        "TransformExecution"
    }

    fn connect_to(&mut self, input: ExecutionRef) -> CSVQueryResult<()> {
        self.input = Some(input);
        Ok(())
    }

    async fn execute(&self) -> CSVQueryResult<SendableDataBlockStream> {
        let p = self.input.clone().unwrap();
        Ok(Box::pin(TransformedStream::new(
            p.execute().await?,
            self.transform_func,
        )))
    }
}
