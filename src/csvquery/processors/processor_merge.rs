use crate::csvquery::data_block::DataBlock;
use crate::csvquery::data_streams::{DataBlockStream, ChannelStream};
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::processors::{IProcessor, ProcessorRef};
use futures::StreamExt;
use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct MergeProcessor {
    inputs: Vec<ProcessorRef>,
}

impl MergeProcessor {
    pub fn new() -> Self {
        Self { inputs: vec![] }
    }
}

#[async_trait]
impl IProcessor for MergeProcessor {
    fn name(&self) -> &'static str {
        "MergeProcessor"
    }

    fn connect_to(&mut self, input: ProcessorRef) -> CSVQueryResult<()> {
        self.inputs.push(input);
        Ok(())
    }

    async fn execute(&self) -> CSVQueryResult<DataBlockStream> {
        let num_inputs = self.inputs.len();
        match num_inputs {
            0 => Err(CSVQueryError::Internal(
                "Can not merge empty processor list".to_string(),
            )),
            1 => self.inputs[0].execute().await,
            _ => {
                let (sender, receiver) = mpsc::channel::<CSVQueryResult<DataBlock>>(num_inputs);
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