use crate::csvquery::data_types::ColumnVector;
use crate::csvquery::error::CQResult;
use arrow::array::ArrayRef;
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

pub type PhysicalExprRef = Arc<dyn PhysicalExpr>;

pub trait PhysicalExpr {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ColumnVector>;
}
