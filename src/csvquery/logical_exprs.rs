use std::sync::Arc;
use crate::csvquery::data_schema::DataField;

pub type LogicalExprRef = Arc<Box<dyn ILogicalExpr>>;

pub trait ILogicalExpr {
    fn to_field(&self) -> DataField;
}

