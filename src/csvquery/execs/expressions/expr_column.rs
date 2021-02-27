use crate::csvquery::data_types::ColumnVector;
use crate::csvquery::error::CQResult;
use crate::csvquery::execs::expressions::PhysicalExpr;
use arrow::record_batch::RecordBatch;

pub struct Column {
    name: String,
}

impl Column {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl PhysicalExpr for Column {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ColumnVector> {
        Ok(ColumnVector::Array(
            batch.column(batch.schema().index_of(&self.name)?).clone(),
        ))
    }
}
