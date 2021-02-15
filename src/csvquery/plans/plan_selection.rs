use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{PlanExpr, PlanNodeRef};

#[derive(Clone)]
pub struct SelectionPlan {
    pub input: PlanNodeRef,
    pub expr: PlanExpr,
}

impl SelectionPlan {
    pub fn new(input: PlanNodeRef, expr: PlanExpr) -> CSVQueryResult<Self> {
        Ok(Self {
            input: input,
            expr: expr,
        })
    }

    pub fn schema(&self) -> DataSchemaRef {
        self.input.schema().clone()
    }
}
