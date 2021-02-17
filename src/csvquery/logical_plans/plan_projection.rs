use crate::csvquery::data_types::{DataSchema, DataSchemaRef};
use crate::csvquery::error::CQResult;
use crate::csvquery::logical_plans::{exprs_to_fields, PlanExpr, PlanNodeRef};
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProjectionPlan {
    pub input: PlanNodeRef,
    pub exprs: Vec<PlanExpr>,
}

impl ProjectionPlan {
    pub fn new(input: PlanNodeRef, exprs: Vec<PlanExpr>) -> Self {
        Self {
            input: input,
            exprs: exprs,
        }
    }

    pub fn schema(&self) -> CQResult<DataSchemaRef> {
        let fields = exprs_to_fields(self.input.clone(), &self.exprs)?;
        let schema = Arc::new(DataSchema::new(fields));
        Ok(schema)
    }
}

impl fmt::Display for ProjectionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Projection: ")?;

        let names = self
            .exprs
            .iter()
            .map(|e| format!("{}", e).to_string())
            .collect::<Vec<String>>();
        write!(f, "{}", names.join(", "))?;

        Ok(())
    }
}
