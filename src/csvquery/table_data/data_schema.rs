use std::collections::HashMap;
use super::data_type::DataType;

#[derive(Debug)]
pub struct DataSchema {
    column_names: HashMap<i32, String>,
    column_types: Vec<DataType>,
}
