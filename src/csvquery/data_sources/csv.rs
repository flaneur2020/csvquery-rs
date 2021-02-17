use crate::csvquery::data_sources::DataSource;
use crate::csvquery::data_types::{DataField, DataSchema, DataSchemaRef};
use crate::csvquery::execs::{ExecutionRef};
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::streams::{CsvReadOptions};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;

pub struct CsvDataSource {
    path: String,
    read_options: CsvReadOptions,
}

impl CsvDataSource {
    pub fn try_new(path: &str, read_options: &CsvReadOptions) -> CQResult<Self> {
        Ok(Self {
            path: path.to_string(),
            read_options: read_options.clone(),
        })
    }
}

impl DataSource for CsvDataSource {
    fn name(&self) -> String {
        format!("CSV: {}", self.path).to_string()
    }

    fn schema(&self) -> CQResult<DataSchemaRef> {
        let fields: Vec<DataField> = Vec::new();
        Ok(Arc::new(DataSchema::new(fields)))
    }

    fn scan(&self) -> CQResult<Vec<ExecutionRef>> {
        Err(CQError::Internal("not implemented".to_string()))
    }
}
