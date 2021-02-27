use crate::csvquery::data_types::ColumnVector;
use crate::csvquery::error::CQResult;
use crate::csvquery::execs::expressions::PhysicalExpr;
use crate::csvquery::scalar::ScalarValue;
use arrow::record_batch::RecordBatch;

pub struct Literal {
    value: ScalarValue,
}

impl Literal {
    fn new(value: &ScalarValue) -> Self {
        Self {
            value: value.clone(),
        }
    }
}

impl PhysicalExpr for Literal {
    fn evaluate(&self, _batch: &RecordBatch) -> CQResult<ColumnVector> {
        Ok(ColumnVector::Scalar(self.value.clone()))
    }
}
