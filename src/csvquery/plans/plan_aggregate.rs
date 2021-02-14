use crate::csvquery::data_schema::{DataSchema, DataSchemaRef};
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{exprs_to_fields, PlanExpr, PlanNodeRef};
use std::sync::Arc;

pub struct AggregatePlan {
    pub input: PlanNodeRef,
    group_exprs: Vec<PlanExpr>,
    aggregate_exprs: Vec<PlanExpr>,
    schema: DataSchemaRef,
}

impl AggregatePlan {
    pub fn new(
        input: PlanNodeRef,
        group_exprs: Vec<PlanExpr>,
        aggregate_exprs: Vec<PlanExpr>,
    ) -> CSVQueryResult<Self> {
        let group_fields = exprs_to_fields(input.clone(), &group_exprs)?;
        let mut aggregate_fields = exprs_to_fields(input.clone(), &aggregate_exprs)?;
        let mut all_fields = group_fields;
        all_fields.append(&mut aggregate_fields);
        let schema = Arc::new(DataSchema::new(all_fields));

        return Ok(Self {
            input: input,
            group_exprs: group_exprs,
            aggregate_exprs: aggregate_exprs,
            schema: schema,
        });
    }

    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}
