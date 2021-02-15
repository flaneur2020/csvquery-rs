use crate::csvquery::data_sources::DataSource;
use crate::csvquery::data_streams::{SendableDataBlockStream, EmptyStream};
use crate::csvquery::data_types::{DataField, DataSchema, DataSchemaRef, DataBlock};
use crate::csvquery::error::{CQResult, CQError};
use std::sync::Arc;
use std::pin::Pin;
use futures::{Stream, StreamExt};

pub struct CSVDataSource {
    path: String,
}

impl CSVDataSource {
    pub fn try_new(path: &str) -> CQResult<Self> {
        Ok(Self {
            path: path.to_string(),
        })
    }
}

impl DataSource for CSVDataSource {
    fn name(&self) -> String {
        format!("CSV: {}", self.path).to_string()
    }

    fn schema(&self) -> CQResult<DataSchemaRef> {
        let fields: Vec<DataField> = Vec::new();
        Ok(Arc::new(DataSchema::new(fields)))
    }

    fn streams(self) -> CQResult<Vec<SendableDataBlockStream>> {
        Err(CQError::Internal("not implemented".to_string()))
    }
}
