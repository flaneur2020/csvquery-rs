use crate::csvquery::data_types::{DataSchemaRef, RecordBatch};
use crate::csvquery::error::{CQError, CQResult};
use arrow::csv;
use futures::{Stream, StreamExt};
use std::fs::File;
use std::task::{Context, Poll};

#[derive(Debug, Clone)]
pub struct CsvReadOptions {
    pub has_header: bool,
    pub schema: Option<DataSchemaRef>,
    pub batch_size: usize,
    pub delimiter: u8,
}

impl CsvReadOptions {
    pub fn new() -> Self {
        Self {
            has_header: false,
            schema: None,
            batch_size: 512,
            delimiter: b',',
        }
    }

    pub fn has_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }

    pub fn schema(mut self, schema: DataSchemaRef) -> Self {
        self.schema = Some(schema);
        self
    }
}

pub struct CsvStream {
    reader: csv::Reader<File>,
}

impl CsvStream {
    pub fn try_new(
        filename: &str,
        schema: DataSchemaRef,
        projection: Option<Vec<usize>>,
        read_options: CsvReadOptions,
    ) -> CQResult<Self> {
        let file = File::open(filename)?;
        let reader = csv::Reader::new(
            file,
            schema,
            read_options.has_header,
            Some(read_options.delimiter),
            read_options.batch_size,
            None,
            projection,
        );

        Ok(Self { reader })
    }
}

impl Stream for CsvStream {
    type Item = CQResult<RecordBatch>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let o = self.reader.next();

        Poll::Ready(o.map(|r| r.or_else(|err| Err(CQError::from(err)))))
    }
}
