use crate::csvquery::data_block::DataArrayRef;
use crate::csvquery::data_schema::DataSchemaRef;

#[derive(Debug, Clone)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
}
