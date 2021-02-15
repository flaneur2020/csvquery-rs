use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::data_block::DataBlock;
use futures::{Stream, StreamExt};
use std::task::{Context, Poll};

pub struct EmptyStream {
}

impl Stream for EmptyStream {
    type Item = CSVQueryResult<DataBlock>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Poll::Pending
    }
}