use crate::csvquery::data_streams::SendableDataBlockStream;
use crate::csvquery::data_types::DataBlock;
use crate::csvquery::error::CSVQueryResult;
use futures::{Stream, StreamExt};
use std::task::{Context, Poll};

pub type TransformFunc = fn(DataBlock) -> CSVQueryResult<DataBlock>;

pub struct TransformedStream {
    input: SendableDataBlockStream,
    func: TransformFunc,
}

impl TransformedStream {
    pub fn new(input: SendableDataBlockStream, func: TransformFunc) -> Self {
        Self { input, func }
    }
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
