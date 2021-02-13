use std::sync::Arc;
use super::data_array::DataArrayRef;
use crate::csvquery::data_schema::DataSchemaRef;

pub type DataBlockRef = Arc<DataBlock>;

#[derive(Debug)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
    rows_count: u64,
}