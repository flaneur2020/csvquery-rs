use std::fs::File;
use arrow::csv;
use futures::{Stream, StreamExt};
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::data_types::{DataSchemaRef}

pub struct CsvStream {
    reader: csv::Reader;
}

impl CsvStream {
    fn try_new(
        filename: &str,
        schema: DataSchemaRef,
        has_header: bool,
        batch_size: usize,
        delimiter: Option<u8>,
        projection: &Option<Vec<usize>>,
    ) -> CSVQueryResult<Self> {
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

        Self { reader }
    }
}

impl Stream for CsvStream {
    type Item = CSVQueryResult<DataBlock>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Ready(self.reader.next())
    }
}