use std::sync::Arc;
use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::data_sources::DataSourceRef;
use crate::csvquery::logical_exprs::LogicalExpr;

pub type LogicalPlanRef = Arc<Box<dyn ILogicalPlan>>;

pub trait ILogicalPlan {
    fn schema(&self) -> DataSchemaRef;
}

pub struct Scan {
    path: String,
    data_source: DataSourceRef,
    projections: Vec<String>,
    schema: DataSchemaRef,
}

impl Scan {
    pub fn new(path: String, data_source: DataSourceRef, projections: Vec<String>) -> Self {
        let schema = Self::derive_schema(data_source.schema().clone(), &projections);
        Self {
            path: path,
            data_source: data_source,
            projections: projections,
            schema: schema,
        }
    }

    fn derive_schema(base_schema: DataSchemaRef, projections: &Vec<String>) -> DataSchemaRef {
        if projections.len() == 0 {
            return base_schema;
        }
        base_schema.select(projections)
    }
}

impl ILogicalPlan for Scan {
    fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}

pub struct Projection {
    input: LogicalPlanRef,
    schema: DataSchemaRef,
}

impl Projection {
    pub fn new(input: LogicalPlanRef, exprs: Vec<LogicalExpr>) {

    }
}