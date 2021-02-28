use crate::csvquery::data_types::{ColumnVector, DataArrayRef};
use crate::csvquery::scalar::ScalarValue;
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::execs::expressions::{PhysicalExpr, PhysicalExprRef};
use crate::csvquery::logical_plans::BinaryExprOP;
use arrow::compute::kernels::comparison::{eq, gt, gt_eq, lt, lt_eq, neq};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

pub struct BinaryExpr {
    op: BinaryExprOP,
    left: PhysicalExprRef,
    right: PhysicalExprRef,
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
                // Ok(ColumnVector::Array(Arc::new(eq(&*ll, &*rr)?)))
                // match l.data_type() {
                //
                // }
                Err(CQError::NotImplemented("Not Implemented".to_string()))
            },
            (ColumnVector::Scalar(ll), ColumnVector::Scalar(rr)) => {
                Ok(ColumnVector::Scalar(ScalarValue::Boolean(Some(ll == rr))))
            },
            (ColumnVector::Array(_), ColumnVector::Scalar(_)) => {
                Err(CQError::NotImplemented("Not Implemented".to_string()))
            },
            (ColumnVector::Scalar(_), ColumnVector::Array(_)) => {
                Err(CQError::NotImplemented("Not Implemented".to_string()))
            },
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
