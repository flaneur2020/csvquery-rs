use crate::csvquery::data_types::{DataArrayRef, DataType};
use arrow::array::{
    new_null_array, BooleanArray, Float32Array, Float64Array, Int8Array, Int16Array, Int32Array, Int64Array,
    StringArray, UInt8Array, UInt16Array, UInt32Array, UInt64Array,
};
use std::iter::repeat;
use std::sync::Arc;

/// Represents a dynamically typed, nullable single value.
/// This is the single-valued counter-part of arrow’s `Array`.
#[derive(Clone, PartialEq)]
pub enum ScalarValue {
    /// true or false value
    Boolean(Option<bool>),
    /// 32bit float
    Float32(Option<f32>),
    /// 64bit float
    Float64(Option<f64>),
    /// signed 8bit int
    Int8(Option<i8>),
    /// signed 16bit int
    Int16(Option<i16>),
    /// signed 32bit int
    Int32(Option<i32>),
    /// signed 64bit int
    Int64(Option<i64>),
    /// unsigned 8bit int
    UInt8(Option<u8>),
    /// unsigned 16bit int
    UInt16(Option<u16>),
    /// unsigned 32bit int
    UInt32(Option<u32>),
    /// unsigned 64bit int
    UInt64(Option<u64>),
    /// utf-8 encoded string.
    Utf8(Option<String>),
}

macro_rules! scalar_value_to_array {
    ($o:expr, $size:expr, $ARRAY_TYPE:ident, $SCALAR:ident) => {
        match $o {
            Some(v) => Arc::new($ARRAY_TYPE::from_iter_values(repeat(*v).take($size))),
            None => new_null_array(&DataType::$SCALAR.into(), $size),
        }
    };
}

impl ScalarValue {
    pub fn data_type(&self) -> DataType {
        match self {
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

    pub fn into_array(&self, size: usize) -> DataArrayRef {
        match self {
            ScalarValue::Boolean(o) => Arc::new(BooleanArray::from(vec![*o; size])) as DataArrayRef,
            ScalarValue::Float32(o) => scalar_value_to_array!(o, size, Float32Array, Float32),
            ScalarValue::Float64(o) => scalar_value_to_array!(o, size, Float64Array, Float64),
            ScalarValue::Int8(o) => scalar_value_to_array!(o, size, Int8Array, Int8),
            ScalarValue::Int16(o) => scalar_value_to_array!(o, size, Int16Array, Int16),
            ScalarValue::Int32(o) => scalar_value_to_array!(o, size, Int32Array, Int32),
            ScalarValue::Int64(o) => scalar_value_to_array!(o, size, Int64Array, Int64),
            ScalarValue::UInt8(o) => scalar_value_to_array!(o, size, UInt8Array, UInt8),
            ScalarValue::UInt16(o) => scalar_value_to_array!(o, size, UInt16Array, UInt16),
            ScalarValue::UInt32(o) => scalar_value_to_array!(o, size, UInt32Array, UInt32),
            ScalarValue::UInt64(o) => scalar_value_to_array!(o, size, UInt64Array, UInt64),
            ScalarValue::Utf8(o) => match o {
                Some(v) => Arc::new(StringArray::from_iter_values(repeat(v).take(size))),
                None => new_null_array(&DataType::Utf8.into(), size),
            },
        }
    }
}
