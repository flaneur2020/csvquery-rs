use crate::csvquery::data_streams::DataBlockStream;
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::processors::{IProcessor, MergeProcessor, ProcessorRef};
use std::sync::Arc;

pub type Pipe = Vec<ProcessorRef>;

pub struct Pipeline {
    processors: Vec<Pipe>,
}

/// pipeline contains a sequence of simple transforms
impl Pipeline {
    pub fn new() -> Self {
        Pipeline { processors: vec![] }
    }

    pub fn add_source(&mut self, processor: ProcessorRef) -> CSVQueryResult<()> {
        if self.processors.len() == 0 {
            self.processors.push(vec![]);
        }
        self.processors[0].push(processor);
        Ok(())
    }

    /// Add a simple transform processor to the pipeline
    ///
    /// processor1 --> processor1_1
    ///
    /// processor2 --> processor2_1
    ///
    /// processor3 --> processor3_1
    ///
    pub fn add_simple_transform(
        &mut self,
        f: impl Fn() -> CSVQueryResult<Box<dyn IProcessor>>,
    ) -> CSVQueryResult<()> {
        let last = self.processors.last().ok_or_else(|| {
            CSVQueryError::Internal("Can't add transform to empty pipeline".to_string())
        })?;
        let mut items: Vec<ProcessorRef> = vec![];
        for x in last {
            let mut p = f()?;
            p.connect_to(x.clone())?;
            items.push(Arc::from(p));
        }
        self.processors.push(items);
        Ok(())
    }

    /// Merge many ways processor into one-way
    ///
    /// processor1 --
    ///               \
    /// processor2 --   --> processor
    ///               /
    /// processor3 --
    pub fn merge_processor(&mut self) -> CSVQueryResult<()> {
        let last = self.processors.last().ok_or_else(|| {
            CSVQueryError::Internal("Can't add transform to empty pipeline".to_string())
        })?;

        if last.len() > 1 {
            let mut p = MergeProcessor::new();
            for x in last {
                p.connect_to(x.clone())?;
            }
            self.processors.push(vec![Arc::new(p)]);
        }
        Ok(())
    }

    pub async fn execute(&mut self) -> CSVQueryResult<DataBlockStream> {
        let last = self
            .processors
            .last()
            .ok_or_else(|| CSVQueryError::Internal("Can't execute empty pipeline".to_string()))?;

        if last.len() > 1 {
            self.merge_processor()?;
        }

        let p = &(self.processors.last().unwrap()[0]);
        p.execute().await
    }
}
