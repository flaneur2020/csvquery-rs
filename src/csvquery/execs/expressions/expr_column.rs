use arrow::array::ArrayRef;
use arrow::record_batch::RecordBatch;
use crate::csvquery::execs::expressions::PhysicalExpr;
use crate::csvquery::error::CQResult;

pub struct Column {
    name: String
}

impl Column {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl PhysicalExpr for Column {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ArrayRef> {
        Ok(batch.column(batch.schema().index_of(&self.name)?).clone())
    }
}