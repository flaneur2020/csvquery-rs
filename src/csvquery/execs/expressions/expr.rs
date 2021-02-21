use arrow::record_batch::RecordBatch;
use arrow::array::ArrayRef;
use crate::csvquery::error::CQResult;
use crate::csvquery::data_types::ColumnVector;

pub trait PhysicalExpr {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ColumnVector>;
}