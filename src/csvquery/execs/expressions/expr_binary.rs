use arrow::record_batch::RecordBatch;
use crate::csvquery::execs::expressions::PhysicalExpr;
use crate::csvquery::error::CQResult;
use crate::csvquery::data_types::ColumnVector;

pub struct BinaryExpr {
    name: String
}

impl BinaryExpr {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl PhysicalExpr for BinaryExpr {
    fn evaluate(&self, batch: &RecordBatch) -> CQResult<ColumnVector> {
        Ok(ColumnVector::Array(batch.column(batch.schema().index_of(&self.name)?).clone()))
    }
}