use crate::csvquery::data_block::DataArrayRef;
use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::error::CSVQueryResult;
use tokio::sync::mpsc::Receiver;

pub type DataBlockChannel = Receiver<CSVQueryResult<DataBlock>>;

#[derive(Debug, Clone)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
}
