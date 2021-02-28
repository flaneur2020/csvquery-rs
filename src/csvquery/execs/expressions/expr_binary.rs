use crate::csvquery::data_types::{ArrayRef, ColumnVector, DataType};
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::execs::expressions::{PhysicalExpr, PhysicalExprRef};
use crate::csvquery::logical_plans::BinaryExprOP;
use crate::csvquery::scalar::ScalarValue;
use arrow::array::{
    BooleanArray, Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array,
    NullArray, UInt16Array, UInt32Array, UInt64Array, UInt8Array, StringArray,
};
use arrow::compute::kernels::comparison::{eq, gt, gt_eq, lt, lt_eq, neq};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

pub struct BinaryExpr {
    op: BinaryExprOP,
    left: PhysicalExprRef,
    right: PhysicalExprRef,
}

macro_rules! compute_op_with_data_type {
    ($L:expr, $R:expr, $OP:ident, $DT:ident) => {{
        let ll = $L
            .as_any()
            .downcast_ref::<$DT>()
            .expect("compute_op failed to downcast array");
        let rr = $R
            .as_any()
            .downcast_ref::<$DT>()
            .expect("compute_op failed to downcast array");
        Arc::new($OP(ll, rr)?)
    }};
}

macro_rules! compute_op_array {
    ($L:expr, $R:expr, $OP:ident) => {{
        match DataType::from($L.data_type().clone()) {
            // DataType::Boolean => compute_op_with_data_type!($L, $R, $OP, BooleanArray),
            DataType::Int8 => compute_op_with_data_type!($L, $R, $OP, Int8Array),
            DataType::Int16 => compute_op_with_data_type!($L, $R, $OP, Int16Array),
            DataType::Int32 => compute_op_with_data_type!($L, $R, $OP, Int32Array),
            DataType::Int64 => compute_op_with_data_type!($L, $R, $OP, Int64Array),
            DataType::UInt8 => compute_op_with_data_type!($L, $R, $OP, UInt8Array),
            DataType::UInt16 => compute_op_with_data_type!($L, $R, $OP, UInt16Array),
            DataType::UInt32 => compute_op_with_data_type!($L, $R, $OP, UInt32Array),
            DataType::UInt64 => compute_op_with_data_type!($L, $R, $OP, UInt64Array),
            DataType::Float32 => compute_op_with_data_type!($L, $R, $OP, Float32Array),
            DataType::Float64 => compute_op_with_data_type!($L, $R, $OP, Float64Array),
            // DataType::Utf8 => compute_op_with_data_type!($L, $R, $OP, StringArray),
            _ => panic!("not implemented"),
        }
    }};
}

impl BinaryExpr {
    pub fn new(op: BinaryExprOP, left: PhysicalExprRef, right: PhysicalExprRef) -> Self {
        Self {
            op: op,
            left: left,
            right: right,
        }
    }

    fn evaluate_eq(&self, l: &ColumnVector, r: &ColumnVector) -> CQResult<ColumnVector> {
        match (l, r) {
            (ColumnVector::Array(ll), ColumnVector::Array(rr)) => {
                compute_op_array!(ll, rr, eq);
                Err(CQError::NotImplemented("Not Implemented".to_string()))
            }
            (ColumnVector::Scalar(ll), ColumnVector::Scalar(rr)) => {
                Ok(ColumnVector::Scalar(ScalarValue::Boolean(Some(ll == rr))))
            }
            (ColumnVector::Array(_), ColumnVector::Scalar(_)) => {
                Err(CQError::NotImplemented("Not Implemented".to_string()))
            }
            (ColumnVector::Scalar(_), ColumnVector::Array(_)) => {
                Err(CQError::NotImplemented("Not Implemented".to_string()))
            }
        }
    }
}

// macro_rules! binary_array_op {}

impl PhysicalExpr for BinaryExpr {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ColumnVector> {
        let ll = self.left.evaluate(batch)?;
        let rr = self.right.evaluate(batch)?;
        if ll.data_type() != rr.data_type() {
            return Err(CQError::Internal(format!(
                "type mismatch {} != {}",
                ll.data_type(),
                rr.data_type()
            )));
        }

        match self.op {
            BinaryExprOP::Eq => self.evaluate_eq(&ll, &rr),
            _ => Err(CQError::NotImplemented("Not Implemented".to_string())),
        }
    }
}
