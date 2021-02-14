use crate::csvquery::data_block::DataBlock;
use crate::csvquery::data_streams::{DataBlockStream, TransformedStream, TransformFunc};
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::processors::{IProcessor, ProcessorRef};
use async_trait::async_trait;

pub struct TransformProcessor {
    input: Option<ProcessorRef>,
    transform_name: &'static str,
    transform_func: TransformFunc,
}

impl TransformProcessor {
    pub fn new(transform_name: &'static str, transform_func: TransformFunc) -> Self {
        Self { input: None, transform_name, transform_func }
    }
}

#[async_trait]
impl IProcessor for TransformProcessor {
    fn name(&self) -> &'static str {
        format!("TransformProcessor<{}>", self.transform_name)
    }

    fn connect_to(&mut self, input: ProcessorRef) -> CSVQueryResult<()> {
        self.input = Some(input);
        Ok(())
    }

    async fn execute(&self) -> CSVQueryResult<DataBlockStream> {
        let p = self.input.clone().unwrap();
        Ok(Box::pin(TransformedStream::new(p.execute().await?, self.transform_func)))
    }
}