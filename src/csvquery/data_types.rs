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
            ColumnVector::Array(arr) => self.array_data_type(arr.clone()),
            ColumnVector::Scalar(s) => self.scalar_data_type(s),
        }
    }

    fn array_data_type(&self, arr: ArrayRef) -> DataType {
        arr.data_type().clone()
    }

    fn scalar_data_type(&self, s: &ScalarValue) -> DataType {
        match s {
            ScalarValue::Boolean(_) => DataType::Boolean,
            ScalarValue::Float32(_) => DataType::Float32,
            ScalarValue::Float64(_) => DataType::Float64,
            ScalarValue::Int8(_) => DataType::Int8,
            ScalarValue::Int16(_) => DataType::Int16,
            ScalarValue::Int32(_) => DataType::Int32,
            ScalarValue::Int64(_) => DataType::Int64,
            ScalarValue::UInt8(_) => DataType::UInt8,
            ScalarValue::UInt16(_) => DataType::UInt16,
            ScalarValue::UInt32(_) => DataType::UInt32,
            ScalarValue::UInt64(_) => DataType::UInt64,
            ScalarValue::Utf8(_) => DataType::Utf8,
        }
    }
}