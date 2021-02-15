use crate::csvquery::data_streams::{SendableDataBlockStream, TransformFunc, TransformedStream};
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::processors::{IProcessor, ProcessorRef};
use async_trait::async_trait;

pub struct TransformProcessor {
    input: Option<ProcessorRef>,
    transform_name: &'static str,
    transform_func: TransformFunc,
}

impl TransformProcessor {
    pub fn new(transform_name: &'static str, transform_func: TransformFunc) -> Self {
        Self {
            input: None,
            transform_name,
            transform_func,
        }
    }
}

#[async_trait]
impl IProcessor for TransformProcessor {
    fn name(&self) -> &'static str {
        "TransformProcessor"
    }

    fn connect_to(&mut self, input: ProcessorRef) -> CSVQueryResult<()> {
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
