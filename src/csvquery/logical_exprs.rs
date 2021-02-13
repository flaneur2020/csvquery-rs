use std::sync::Arc;
use crate::csvquery::error::{CSVQueryResult, CSVQueryError};
use crate::csvquery::data_schema::{DataField, DataType};
use crate::csvquery::logical_plans::LogicalPlanRef;

pub type LogicalExprRef = Arc<Box<dyn ILogicalExpr>>;

pub trait ILogicalExpr {
    fn to_field(&self, input: LogicalPlanRef) -> CSVQueryResult<DataField>;
}

pub struct ColumnExpr {
    name: String,
}

impl ILogicalExpr for ColumnExpr {
    fn to_field(&self, input: LogicalPlanRef) -> CSVQueryResult<DataField> {
        let schema = input.schema().clone();
        let field = schema
            .find_field(&self.name)
            .ok_or_else(|| CSVQueryError::FieldNotFound(self.name.clone()))?;
        Ok(field.clone())
    }
}

pub struct LiteralStringExpr {
    str: String,
}

impl ILogicalExpr for LiteralStringExpr {
    fn to_field(&self, _: LogicalPlanRef) -> CSVQueryResult<DataField> {
        return Ok(DataField::new(self.str.clone(), DataType::String));
    }
}