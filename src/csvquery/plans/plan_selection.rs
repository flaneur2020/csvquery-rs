use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{PlanExpr, PlanNodeRef};

pub struct SelectionPlan {
    input: PlanNodeRef,
    expr: PlanExpr,
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
