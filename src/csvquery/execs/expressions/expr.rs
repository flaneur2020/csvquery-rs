use arrow::record_batch::RecordBatch;
use arrow::array::ArrayRef;
use crate::csvquery::error::CQResult;

pub trait PhysicalExpr {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ArrayRef>;
}