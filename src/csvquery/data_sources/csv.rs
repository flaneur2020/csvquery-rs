use crate::csvquery::data_types::{DataField, DataSchema, DataSchemaRef};
use crate::csvquery::data_sources::DataSource;
use crate::csvquery::data_streams::{DataBlockStream, EmptyStream};
use crate::csvquery::error::CSVQueryResult;
use std::sync::Arc;

pub struct CSVDataSource {
    path: String,
}

impl CSVDataSource {
    pub fn open(path: &str) -> CSVQueryResult<Self> {
        Ok(Self { path: path.to_string() })
    }
}

impl DataSource for CSVDataSource {
    fn schema(&self) -> DataSchemaRef {
        let fields: Vec<DataField> = Vec::new();
        Arc::new(DataSchema::new(fields))
    }

    fn stream(&self) -> DataBlockStream {
        Box::pin(EmptyStream {})
    }
}