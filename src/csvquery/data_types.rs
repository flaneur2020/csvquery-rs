use crate::csvquery::scalar::ScalarValue;
use arrow;
use arrow::array::ArrayRef;

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

impl ColumnVector {
    pub fn data_type(&self) -> DataType {
        match self {
            ColumnVector::Array(arr) => arr.data_type().clone(),
            ColumnVector::Scalar(s) => s.data_type(),
        }
    }

    pub fn into_array(&self, size: usize) -> ArrayRef {
        match self {
            ColumnVector::Array(arr) => arr.clone(),
            ColumnVector::Scalar(scalar) => scalar.into_array(size),
        }
    }
}
