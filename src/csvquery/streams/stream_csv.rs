use crate::csvquery::data_types::{DataBlock, DataSchemaRef};
use crate::csvquery::error::{CQError, CQResult};
use arrow::csv;
use futures::{Stream, StreamExt};
use std::fs::File;
use std::task::{Context, Poll};

pub struct CsvStream {
    reader: csv::Reader<File>,
}

impl CsvStream {
    fn try_new(
        filename: &str,
        schema: DataSchemaRef,
        has_header: bool,
        batch_size: usize,
        delimiter: Option<u8>,
        projection: &Option<Vec<usize>>,
    ) -> CQResult<Self> {
        let file = File::open(filename)?;
        let reader = csv::Reader::new(
            file,
            schema,
            has_header,
            delimiter,
            batch_size,
            None,
            projection.clone(),
        );

        Ok(Self { reader })
    }
}

impl Stream for CsvStream {
    type Item = CQResult<DataBlock>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let o = self.reader.next();
        if o.is_none() {
            return Poll::Ready(None);
        }

        Poll::Ready(Some(match o.unwrap() {
            Ok(batch) => DataBlock::from_arrow_record_batch(&batch),
            Err(err) => Err(CQError::from(err)),
        }))
    }
}
