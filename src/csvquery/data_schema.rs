use std::sync::Arc;
use std::collections::HashMap;
use crate::csvquery::error::CSVQueryError;

#[derive(Debug, Copy, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
}

pub type DataSchemaRef = Arc<DataSchema>;

#[derive(Debug)]
pub struct DataSchema {
    columns: HashMap<String, DataType>,
}

impl DataSchema {
    pub fn new(columns: HashMap<String, DataType>) -> Self {
        return Self {
            columns: columns,
        }
    }

    pub fn select(&self, projections: &Vec<String>) -> DataSchemaRef {
        let mut new_columns: HashMap<String, DataType> = HashMap::new();
        for projection in projections.iter() {
            let data_type = self.columns.get(projection).unwrap();
            new_columns.insert(projection.clone(), data_type.clone());
        }
        let new_schema = DataSchema::new(new_columns);
        return Arc::new(new_schema);
    }
}