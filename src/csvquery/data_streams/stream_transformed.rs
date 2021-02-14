use crate::csvquery::data_block::DataBlock;
use crate::csvquery::data_streams::DataBlockStream;
use crate::csvquery::error::CSVQueryResult;
use futures::{Stream, StreamExt};
use std::task::{Context, Poll};

pub type TransformFunc = fn(DataBlock) -> CSVQueryResult<DataBlock>;

pub struct TransformedStream {
    input: DataBlockStream,
    func: TransformFunc,
}

impl Stream for TransformedStream {
    type Item = CSVQueryResult<DataBlock>;

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
