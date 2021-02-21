use crate::csvquery::data_types::RecordBatch;
use crate::csvquery::error::CQResult;
use futures::stream::Stream;
use std::task::{Context, Poll};
use tokio::sync::mpsc::Receiver;

pub struct ChannelStream {
    pub input: Receiver<CQResult<RecordBatch>>,
}

impl ChannelStream {
    pub fn new(input: Receiver<CQResult<RecordBatch>>) -> Self {
        Self { input }
    }
}

impl Stream for ChannelStream {
    type Item = CQResult<RecordBatch>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.input.poll_recv(cx)
    }
}
