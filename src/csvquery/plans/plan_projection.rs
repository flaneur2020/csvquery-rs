use crate::csvquery::data_types::{DataSchema, DataSchemaRef};
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{exprs_to_fields, PlanExpr, PlanNodeRef};
use std::sync::Arc;
use std::fmt;

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

    pub fn schema(&self) -> CSVQueryResult<DataSchemaRef> {
        let fields = exprs_to_fields(self.input.clone(), &self.exprs)?;
        let schema = Arc::new(DataSchema::new(fields));
        Ok(schema)
    }
}

impl fmt::Display for ProjectionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Projection: ")?;

        match self.schema() {
            Ok(schema) => {
                let field_names = schema.fields()
                    .iter()
                    .map(|f| f.name().to_string() )
                    .collect::<Vec<String>>();
                write!(f, "{}", field_names.join(", "))?;
            }
            Err(err) => {
                write!(f, "Err {}", err);
            }
       }
        Ok(())
    }
}