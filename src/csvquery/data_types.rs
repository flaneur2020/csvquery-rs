use arrow;
use arrow::array::ArrayRef;
use crate::csvquery::scalar::ScalarValue;

pub type DataType = arrow::datatypes::DataType;
pub type DataField = arrow::datatypes::Field;
pub type DataSchemaRef = arrow::datatypes::SchemaRef;
pub type DataSchema = arrow::datatypes::Schema;
pub type DataArrayRef = arrow::array::ArrayRef;
pub type RecordBatch = arrow::record_batch::RecordBatch;

pub enum ColumnVector {
    Array(ArrayRef),
    Scalar(ScalarValue),
}