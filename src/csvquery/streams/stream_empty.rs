use crate::csvquery::data_types::RecordBatch;
use crate::csvquery::error::CQResult;
use futures::{Stream, StreamExt};
use std::task::{Context, Poll};

pub struct EmptyStream {}

impl Stream for EmptyStream {
    type Item = CQResult<RecordBatch>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Poll::Pending
    }
}
