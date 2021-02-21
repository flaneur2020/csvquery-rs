use crate::csvquery::error::CQResult;
use arrow;
use arrow::array::ArrayRef;
use crate::csvquery::scalar::ScalarValue;

pub type DataType = arrow::datatypes::DataType;
pub type DataField = arrow::datatypes::Field;
pub type DataSchemaRef = arrow::datatypes::SchemaRef;
pub type DataSchema = arrow::datatypes::Schema;
pub type DataArrayRef = arrow::array::ArrayRef;

#[derive(Debug, Clone)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
}

impl DataBlock {
    pub fn new(schema: DataSchemaRef, columns: Vec<DataArrayRef>) -> Self {
        Self { schema, columns }
    }

    pub fn from_arrow_record_batch(
        batch: &arrow::record_batch::RecordBatch,
    ) -> CQResult<DataBlock> {
        Ok(DataBlock::new(batch.schema(), Vec::from(batch.columns())))
    }

    pub fn to_arrow_record_batch(&self) -> CQResult<arrow::record_batch::RecordBatch> {
        Ok(arrow::record_batch::RecordBatch::try_new(
            self.schema.clone(),
            self.columns.clone(),
        )?)
    }
}

pub enum ColumnVector {
    Array(ArrayRef),
    Scalar(ScalarValue),
}