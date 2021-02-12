use crate::csvquery::datablock::{DataBlockRef, DataSchemaRef};
use crate::csvquery::error::CSVQueryError;

pub trait IDataTable {
    fn data_block(&self, n: int) -> Result<DataBlockRef, CSVQueryError>;

    fn data_schema() -> DataSchemaRef;
}