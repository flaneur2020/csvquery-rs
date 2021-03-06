use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::error::CQResult;
use crate::csvquery::logical_plans::{PlanExpr, PlanNodeRef};
use std::fmt;

#[derive(Clone)]
pub struct SelectionPlan {
    pub input: PlanNodeRef,
    pub expr: PlanExpr,
}

impl SelectionPlan {
    pub fn new(input: PlanNodeRef, expr: PlanExpr) -> Self {
        Self {
            input: input,
            expr: expr,
        }
    }

    pub fn schema(&self) -> CQResult<DataSchemaRef> {
        Ok(self.input.schema()?.clone())
    }
}

impl fmt::Display for SelectionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Selection: {}", self.expr)?;
        Ok(())
    }
}
