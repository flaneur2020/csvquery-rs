use std::sync::Arc;
use std::collections::HashMap;

pub type DataSchemaRef = Arc<DataSchema>;

#[derive(Debug)]
pub struct DataSchema {
    column_names: HashMap<i32, String>,
    column_types: Vec<DataType>,
}

#[derive(Debug)]
pub enum DataType {
    Int,
    Float,
    String,
}