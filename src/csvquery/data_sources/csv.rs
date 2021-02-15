use crate::csvquery::data_sources::DataSource;
use crate::csvquery::data_streams::{SendableDataBlockStream, EmptyStream};
use crate::csvquery::data_types::{DataField, DataSchema, DataSchemaRef, DataBlock};
use crate::csvquery::error::{CSVQueryResult, CSVQueryError};
use std::sync::Arc;
use std::pin::Pin;
use futures::{Stream, StreamExt};

pub struct CSVDataSource {
    path: String,
}

impl CSVDataSource {
    pub fn try_new(path: &str) -> CSVQueryResult<Self> {
        Ok(Self {
            path: path.to_string(),
        })
    }
}

impl DataSource for CSVDataSource {
    fn name(&self) -> String {
        format!("CSV: {}", self.path).to_string()
    }

    fn schema(&self) -> CSVQueryResult<DataSchemaRef> {
        let fields: Vec<DataField> = Vec::new();
        Ok(Arc::new(DataSchema::new(fields)))
    }

    fn streams(self) -> CSVQueryResult<Vec<SendableDataBlockStream>> {
        Err(CSVQueryError::Internal("not implemented".to_string()))
    }
}
