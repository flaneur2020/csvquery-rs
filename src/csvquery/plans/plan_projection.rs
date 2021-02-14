use crate::csvquery::data_schema::{DataSchema, DataSchemaRef};
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{exprs_to_fields, PlanExpr, PlanNodeRef};
use std::sync::Arc;

pub struct ProjectionPlan {
    pub input: PlanNodeRef,
    schema: DataSchemaRef,
}

impl ProjectionPlan {
    pub fn new(input: PlanNodeRef, exprs: Vec<PlanExpr>) -> CSVQueryResult<Self> {
        let fields = exprs_to_fields(input.clone(), &exprs)?;
        let schema = Arc::new(DataSchema::new(fields));

        Ok(Self {
            input: input,
            schema: schema,
        })
    }

    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}
