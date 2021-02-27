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
    pub fn new(
        op: BinaryExprOP,
        left: PhysicalExprRef,
        right: PhysicalExprRef,
    ) -> Self {
        Self {
            op: op,
            left: left,
            right: right,
        }
    }

    fn evaluate_eq(&self, batch: &RecordBatch) -> CQResult<ColumnVector> {
        Err(CQError::NotImplemented("Not Implemented".to_string()))
    }
}

impl PhysicalExpr for BinaryExpr {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ColumnVector> {
        let ll = self.left.evaluate(batch)?;
        let rr = self.right.evaluate(batch)?;
        match self.op {
            BinaryExprOP::Eq => self.evaluate_eq(batch),
            _ => Err(CQError::NotImplemented("Not Implemented".to_string())),
        }
    }
}
