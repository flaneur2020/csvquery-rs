use crate::csvquery::scalar::ScalarValue;
use arrow;
use std::fmt;

// pub type DataType = arrow::datatypes::DataType;
pub type DataField = arrow::datatypes::Field;
pub type DataSchemaRef = arrow::datatypes::SchemaRef;
pub type DataSchema = arrow::datatypes::Schema;
pub type ArrayRef = arrow::array::ArrayRef;
pub type RecordBatch = arrow::record_batch::RecordBatch;

// a sub set of arrow's DataType
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataType {
    /// Null type
    Null,
    /// A boolean datatype representing the values `true` and `false`.
    Boolean,
    /// A signed 8-bit integer.
    Int8,
    /// A signed 16-bit integer.
    Int16,
    /// A signed 32-bit integer.
    Int32,
    /// A signed 64-bit integer.
    Int64,
    /// An unsigned 8-bit integer.
    UInt8,
    /// An unsigned 16-bit integer.
    UInt16,
    /// An unsigned 32-bit integer.
    UInt32,
    /// An unsigned 64-bit integer.
    UInt64,
    /// A 32-bit floating point number.
    Float32,
    /// A 64-bit floating point number.
    Float64,
    /// A variable-length string in Unicode with UTF-8 encoding.
    Utf8,
}

impl From<arrow::datatypes::DataType> for DataType {
    fn from(dt: arrow::datatypes::DataType) -> Self {
        match dt {
            arrow::datatypes::DataType::Null => DataType::Null,
            arrow::datatypes::DataType::Boolean => DataType::Boolean,
            arrow::datatypes::DataType::Int8 => DataType::Int8,
            arrow::datatypes::DataType::Int16 => DataType::Int16,
            arrow::datatypes::DataType::Int32 => DataType::Int32,
            arrow::datatypes::DataType::Int64 => DataType::Int64,
            arrow::datatypes::DataType::UInt8 => DataType::UInt8,
            arrow::datatypes::DataType::UInt16 => DataType::UInt16,
            arrow::datatypes::DataType::UInt32 => DataType::UInt32,
            arrow::datatypes::DataType::UInt64 => DataType::UInt64,
            arrow::datatypes::DataType::Float32 => DataType::Float32,
            arrow::datatypes::DataType::Float64 => DataType::Float64,
            arrow::datatypes::DataType::Utf8 => DataType::Utf8,
            _ => panic!("unsuported arrow data type: {}", dt),
        }
    }
}

impl From<DataType> for arrow::datatypes::DataType {
    fn from(dt: DataType) -> arrow::datatypes::DataType {
        match dt {
            DataType::Null => arrow::datatypes::DataType::Null,
            DataType::Boolean => arrow::datatypes::DataType::Boolean,
            DataType::Int8 => arrow::datatypes::DataType::Int8,
            DataType::Int16 => arrow::datatypes::DataType::Int16,
            DataType::Int32 => arrow::datatypes::DataType::Int32,
            DataType::Int64 => arrow::datatypes::DataType::Int64,
            DataType::UInt8 => arrow::datatypes::DataType::UInt8,
            DataType::UInt16 => arrow::datatypes::DataType::UInt16,
            DataType::UInt32 => arrow::datatypes::DataType::UInt32,
            DataType::UInt64 => arrow::datatypes::DataType::UInt64,
            DataType::Float32 => arrow::datatypes::DataType::Float32,
            DataType::Float64 => arrow::datatypes::DataType::Float64,
            DataType::Utf8 => arrow::datatypes::DataType::Float64,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

pub enum ColumnVector {
    Array(ArrayRef),
    Scalar(ScalarValue),
}

impl ColumnVector {
    pub fn data_type(&self) -> DataType {
        match self {
            ColumnVector::Array(arr) => DataType::from(arr.data_type().clone()),
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
