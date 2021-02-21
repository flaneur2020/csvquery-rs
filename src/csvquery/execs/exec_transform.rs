use crate::csvquery::streams::{SendableRecordBatchStream, TransformFunc, TransformedStream};
use crate::csvquery::error::CQResult;
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

    fn inputs(&self) -> Vec<ExecutionRef> {
        match &self.input {
            None => vec![],
            Some(input) => vec![input.clone()],
        }
    }

    fn connect_to(&mut self, input: ExecutionRef) -> CQResult<()> {
        self.input = Some(input);
        Ok(())
    }

    async fn execute(&self) -> CQResult<SendableRecordBatchStream> {
        let p = self.input.clone().unwrap();
        Ok(Box::pin(TransformedStream::new(
            p.execute().await?,
            self.transform_func,
        )))
    }
}
