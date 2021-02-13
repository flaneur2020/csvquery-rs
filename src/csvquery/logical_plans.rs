use std::sync::Arc;
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::data_schema::{DataSchemaRef, DataSchema, DataField};
use crate::csvquery::data_sources::DataSourceRef;
use crate::csvquery::logical_exprs::LogicalExpr;

pub type LogicalPlanRef = Arc<Box<dyn ILogicalPlan>>;

pub trait ILogicalPlan {
    fn schema(&self) -> DataSchemaRef;
}

pub struct ScanPlan {
    path: String,
    data_source: DataSourceRef,
    projections: Vec<String>,
    schema: DataSchemaRef,
}

impl ScanPlan {
    pub fn new(path: String, data_source: DataSourceRef, projections: Vec<String>) -> CSVQueryResult<Self> {
        let schema = Self::derive_schema(data_source.schema().clone(), &projections);
        Ok(Self {
            path: path,
            data_source: data_source,
            projections: projections,
            schema: schema,
        })
    }

    fn derive_schema(base_schema: DataSchemaRef, projections: &Vec<String>) -> DataSchemaRef {
        if projections.len() == 0 {
            return base_schema;
        }
        base_schema.select(projections)
    }
}

impl ILogicalPlan for ScanPlan {
    fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}

pub struct ProjectionPlan {
    input: LogicalPlanRef,
    schema: DataSchemaRef,
}

impl ProjectionPlan {
    pub fn new(input: LogicalPlanRef, exprs: Vec<LogicalExpr>) -> CSVQueryResult<Self> {
        let fields = exprs.iter()
            .map(|e| e.to_field(input.clone()))
            .collect::<CSVQueryResult<Vec<DataField>>>()?;
        let schema = Arc::new(DataSchema::new(fields));

        Ok(Self {
            input: input,
            schema: schema,
        })
    }
}

impl ILogicalPlan for ProjectionPlan {
    fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}

pub struct SelectionPlan {
    input: LogicalPlanRef,
    expr: LogicalExpr,
}

impl SelectionPlan {
    pub fn new(input: LogicalPlanRef, expr: LogicalExpr) -> CSVQueryResult<Self> {
        Ok(Self {
            input: input,
            expr: expr,
        })
    }
}

impl ILogicalPlan for SelectionPlan {
    fn schema(&self) -> DataSchemaRef {
        self.input.schema().clone()
    }
}