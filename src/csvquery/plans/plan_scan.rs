use crate::csvquery::data_sources::DataSourceRef;
use crate::csvquery::data_types::{DataField, DataSchema, DataSchemaRef};
use crate::csvquery::error::{CQError, CQResult};
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct ScanPlan {
    pub data_source: DataSourceRef,
    pub projections: Vec<String>,
}

impl ScanPlan {
    pub fn new(data_source: DataSourceRef, projections: Vec<String>) -> Self {
        Self {
            data_source: data_source,
            projections: projections,
        }
    }

    fn derive_schema(&self) -> CQResult<DataSchemaRef> {
        let base_schema = self.data_source.schema()?.clone();
        if self.projections.len() == 0 {
            return Ok(base_schema);
        }

        let mut new_fields: Vec<DataField> = Vec::new();
        for projection in self.projections.iter() {
            let field = base_schema
                .field_with_name(projection)
                .or_else(|_| Err(CQError::FieldNotFound(projection.clone())))?;
            new_fields.push(field.clone());
        }

        let new_schema = DataSchema::new(new_fields);
        Ok(Arc::new(new_schema))
    }

    pub fn schema(&self) -> CQResult<DataSchemaRef> {
        self.derive_schema()
    }
}

impl fmt::Display for ScanPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scan: {}", self.data_source.name())?;
        Ok(())
    }
}
