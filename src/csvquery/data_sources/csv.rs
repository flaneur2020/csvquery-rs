use crate::csvquery::data_schema::{DataField, DataSchema, DataSchemaRef};
use crate::csvquery::data_sources::DataSource;
use std::sync::Arc;

pub struct CSVDataSource {
    path: String,
}

impl CSVDataSource {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl DataSource for CSVDataSource {
    fn schema(&self) -> DataSchemaRef {
        let fields: Vec<DataField> = Vec::new();
        Arc::new(DataSchema::new(fields))
    }
}
