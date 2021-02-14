use super::data_array::DataArrayRef;
use crate::csvquery::data_schema::DataSchemaRef;
use std::sync::Arc;

pub type DataBlockRef = Arc<DataBlock>;

#[derive(Debug)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
}
