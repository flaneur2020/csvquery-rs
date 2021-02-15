use crate::csvquery::data_streams::{ChannelStream, SendableDataBlockStream};
use crate::csvquery::data_types::DataBlock;
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::execs::{Execution, ExecutionRef};
use async_trait::async_trait;
use futures::StreamExt;
use tokio::sync::mpsc;

pub struct MergeExecution {
    inputs: Vec<ExecutionRef>,
}

impl MergeExecution {
    pub fn new() -> Self {
        Self { inputs: vec![] }
    }
}

#[async_trait]
impl Execution for MergeExecution {
    fn name(&self) -> &'static str {
        "MergeExecution"
    }

    fn connect_to(&mut self, input: ExecutionRef) -> CQResult<()> {
        self.inputs.push(input);
        Ok(())
    }

    async fn execute(&self) -> CQResult<SendableDataBlockStream> {
        let num_inputs = self.inputs.len();
        match num_inputs {
            0 => Err(CQError::Internal(
                "Can not merge empty processor list".to_string(),
            )),
            1 => self.inputs[0].execute().await,
            _ => {
                let (sender, receiver) = mpsc::channel::<CQResult<DataBlock>>(num_inputs);
                for i in 0..num_inputs {
                    let input = self.inputs[i].clone();
                    let sender = sender.clone();

                    tokio::spawn(async move {
                        let mut stream = match input.execute().await {
                            Err(e) => {
                                sender.send(Err(e)).await.ok();
                                return;
                            }
                            Ok(stream) => stream,
                        };

                        while let Some(item) = stream.next().await {
                            sender.send(item).await.ok();
                        }
                    });
                }

                Ok(Box::pin(ChannelStream::new(receiver)))
            }
        }
    }
}
