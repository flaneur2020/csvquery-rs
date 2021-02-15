use crate::csvquery::data_sources::DataSource;
use crate::csvquery::data_streams::{EmptyStream, SendableDataBlockStream};
use crate::csvquery::data_types::{DataBlock, DataField, DataSchema, DataSchemaRef};
use crate::csvquery::error::{CQError, CQResult};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;

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
