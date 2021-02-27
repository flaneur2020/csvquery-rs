use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::execs::{Execution, ExecutionRef, MergeExecution};
use crate::csvquery::streams::SendableRecordBatchStream;
use std::sync::Arc;

pub type Pipe = Vec<ExecutionRef>;

pub struct Pipeline {
    executions: Vec<Pipe>,
}

/// pipeline contains a sequence of simple transforms
impl Pipeline {
    pub fn new() -> Self {
        Pipeline { executions: vec![] }
    }

    pub fn add_source(&mut self, execution: ExecutionRef) -> CQResult<()> {
        if self.executions.len() == 0 {
            self.executions.push(vec![]);
        }
        self.executions[0].push(execution);
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
        f: impl Fn() -> CQResult<Box<dyn Execution>>,
    ) -> CQResult<()> {
        let last = self.executions.last().ok_or_else(|| {
            CQError::Internal("Can't add transform to empty pipeline".to_string())
        })?;
        let mut items: Vec<ExecutionRef> = vec![];
        for x in last {
            let mut p = f()?;
            p.connect_to(x.clone())?;
            items.push(Arc::from(p));
        }
        self.executions.push(items);
        Ok(())
    }

    /// Merge many ways processor into one-way
    ///
    /// processor1 --
    ///               \
    /// processor2 --   --> processor
    ///               /
    /// processor3 --
    pub fn merge_processor(&mut self) -> CQResult<()> {
        let last = self.executions.last().ok_or_else(|| {
            CQError::Internal("Can't add transform to empty pipeline".to_string())
        })?;

        if last.len() > 1 {
            let mut p = MergeExecution::new();
            for x in last {
                p.connect_to(x.clone())?;
            }
            self.executions.push(vec![Arc::new(p)]);
        }
        Ok(())
    }

    pub async fn execute(&mut self) -> CQResult<SendableRecordBatchStream> {
        let last = self
            .executions
            .last()
            .ok_or_else(|| CQError::Internal("Can't execute empty pipeline".to_string()))?;

        if last.len() > 1 {
            self.merge_processor()?;
        }

        let p = &(self.executions.last().unwrap()[0]);
        p.execute().await
    }
}
