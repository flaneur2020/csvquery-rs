use crate::csvquery::data_types::{DataField, DataSchema, DataSchemaRef};
use crate::csvquery::data_sources::DataSourceRef;
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use std::sync::Arc;
use std::fmt;

#[derive(Clone)]
pub struct ScanPlan {
    pub path: String,
    pub data_source: DataSourceRef,
    pub projections: Vec<String>,
    schema: DataSchemaRef,
}

impl ScanPlan {
    pub fn new(
        path: &str,
        data_source: DataSourceRef,
        projections: Vec<String>,
    ) -> CSVQueryResult<Self> {
        let schema = Self::derive_schema(data_source.schema().clone(), &projections)?;
        Ok(Self {
            path: path.to_string(),
            data_source: data_source,
            projections: projections,
            schema: schema,
        })
    }

    fn derive_schema(
        base_schema: DataSchemaRef,
        projections: &Vec<String>,
    ) -> CSVQueryResult<DataSchemaRef> {
        if projections.len() == 0 {
            return Ok(base_schema);
        }

        let mut new_fields: Vec<DataField> = Vec::new();
        for projection in projections.iter() {
            let field = base_schema
                .field_with_name(projection)
                .or_else(|_| Err(CSVQueryError::FieldNotFound(projection.clone())))?;
            new_fields.push(field.clone());
        }

        let new_schema = DataSchema::new(new_fields);
        Ok(Arc::new(new_schema))
    }

    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}

impl fmt::Display for ScanPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scan: {}", self.path)?;
        Ok(())
    }
}