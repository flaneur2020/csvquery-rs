use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::data_sources::DataSourceRef;
use crate::csvquery::error::CSVQueryResult;

pub trait LogicalPlan {
    fn schema(&self) -> DataSchemaRef;
}

pub struct Scan {
    path: String,
    data_source: DataSourceRef,
    projections: Vec<String>,
}

impl Scan {
    pub fn new(path: String, data_source: DataSourceRef, projections: Vec<String>) -> Self {
        Self {
            path: path,
            data_source: data_source,
            projections: projections,
        }
    }

    fn derive_schema(&self) -> DataSchemaRef {
        let schema = self.data_source.schema();
        if self.projections.len() == 0 {
            return schema.clone();
        }
        schema.select(&self.projections)
    }
}

impl LogicalPlan for Scan {
    fn schema(&self) -> DataSchemaRef {
        self.derive_schema()
    }
}