use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::data_sources::DataSourceRef;
use crate::csvquery::error::CSVQueryResult;

pub struct ScanPlan {
    path: String,
    data_source: DataSourceRef,
    projections: Vec<String>,
    schema: DataSchemaRef,
}

impl ScanPlan {
    pub fn new(
        path: String,
        data_source: DataSourceRef,
        projections: Vec<String>,
    ) -> CSVQueryResult<Self> {
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

    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}
