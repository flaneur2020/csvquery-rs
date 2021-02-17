use crate::csvquery::data_types::{DataSchema, DataSchemaRef};
use crate::csvquery::error::CQResult;
use crate::csvquery::logical_plans::{exprs_to_fields, PlanExpr, PlanNodeRef};
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct AggregatePlan {
    pub input: PlanNodeRef,
    group_exprs: Vec<PlanExpr>,
    aggregate_exprs: Vec<PlanExpr>,
}

impl AggregatePlan {
    pub fn new(
        input: PlanNodeRef,
        group_exprs: Vec<PlanExpr>,
        aggregate_exprs: Vec<PlanExpr>,
    ) -> Self {
        return Self {
            input: input,
            group_exprs: group_exprs,
            aggregate_exprs: aggregate_exprs,
        };
    }

    pub fn schema(&self) -> CQResult<DataSchemaRef> {
        let group_fields = exprs_to_fields(self.input.clone(), &self.group_exprs)?;
        let mut aggregate_fields = exprs_to_fields(self.input.clone(), &self.aggregate_exprs)?;
        let mut all_fields = group_fields;
        all_fields.append(&mut aggregate_fields);
        let schema = DataSchema::new(all_fields);
        Ok(Arc::new(schema))
    }
}

impl fmt::Display for AggregatePlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Aggregate")?;
        Ok(())
    }
}
