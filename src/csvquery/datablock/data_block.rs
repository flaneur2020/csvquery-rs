use std::sync::Arc;
use super::data_array::DataArrayRef;
use super::data_schema::DataSchemaRef;

pub type DataBlockRef = Arc<DataBlock>;

#[derive(Debug)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
    length: u64,
}