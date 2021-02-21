use crate::csvquery::data_types::RecordBatch;
use crate::csvquery::streams::SendableRecordBatchStream;
use crate::csvquery::error::CQResult;
use futures::{Stream, StreamExt};
use std::task::{Context, Poll};

pub type TransformFunc = fn(RecordBatch) -> CQResult<RecordBatch>;

pub struct TransformedStream {
    input: SendableRecordBatchStream,
    func: TransformFunc,
}

impl TransformedStream {
    pub fn new(input: SendableRecordBatchStream, func: TransformFunc) -> Self {
        Self { input, func }
    }
}

impl Stream for TransformedStream {
    type Item = CQResult<RecordBatch>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.input.poll_next_unpin(ctx).map(|x| match x {
            Some(Ok(v)) => Some((self.func)(v)),
            other => other,
        })
    }
}
