use crate::csvquery::data_types::DataBlock;
use crate::csvquery::error::CSVQueryResult;
use futures::stream::Stream;
use std::task::{Context, Poll};
use tokio::sync::mpsc::Receiver;

pub struct ChannelStream {
    pub input: Receiver<CSVQueryResult<DataBlock>>,
}

impl ChannelStream {
    pub fn new(input: Receiver<CSVQueryResult<DataBlock>>) -> Self {
        Self { input }
    }
}

impl Stream for ChannelStream {
    type Item = CSVQueryResult<DataBlock>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.input.poll_recv(cx)
    }
}
