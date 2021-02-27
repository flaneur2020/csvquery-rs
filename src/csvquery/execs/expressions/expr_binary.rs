use crate::csvquery::data_types::ColumnVector;
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::execs::expressions::{PhysicalExpr, PhysicalExprRef};
use crate::csvquery::logical_plans::BinaryExprOP;
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
        Err(CQError::NotImplemented("Not Implemented".to_string()))
    }
}

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
