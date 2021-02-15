use crate::csvquery::data_types::{DataSchema, DataSchemaRef};
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{exprs_to_fields, PlanExpr, PlanNodeRef};
use std::sync::Arc;
use std::fmt;

#[derive(Clone)]
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

impl fmt::Display for ProjectionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Projection: ")?;

        let field_names = self.schema
            .fields()
            .iter()
            .map(|f| f.name().to_string() )
            .collect::<Vec<String>>();
        write!(f, "{}", field_names.join(", "))?;

        Ok(())
    }
}