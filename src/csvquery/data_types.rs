use arrow;
use std::sync::Arc;

pub type DataType = arrow::datatypes::DataType;

pub type DataField = arrow::datatypes::Field;

pub type DataSchemaRef = arrow::datatypes::SchemaRef;

pub type DataSchema = arrow::datatypes::Schema;

pub type DataArrayRef = Arc<DataArray>;

#[derive(Debug)]
pub enum DataArray {
    IntArray(Vec<i64>),
    StringArray(Vec<String>),
    FloatArray(Vec<f64>),
}

#[derive(Debug, Clone)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
}