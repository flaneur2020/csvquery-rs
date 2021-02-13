use std::sync::Arc;
use std::collections::HashMap;
use crate::csvquery::error::CSVQueryError;

#[derive(Debug, Copy, Clone)]
pub enum DataType {
    Int64,
    Float64,
    String,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct DataField {
    pub name: String,
    pub data_type: DataType,
}

impl DataField {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name: name,
            data_type: data_type,
        }
    }
}

pub type DataSchemaRef = Arc<DataSchema>;

#[derive(Debug)]
pub struct DataSchema {
    fields: Vec<DataField>,
}

impl DataSchema {
    pub fn new(fields: Vec<DataField>) -> Self {
        return Self {
            fields: fields,
        }
    }

    pub fn find_field(&self, field_name: &str) -> Option<&DataField> {
        self.fields
            .iter()
            .find(|&f| f.name == *field_name)
    }

    pub fn select(&self, projections: &Vec<String>) -> DataSchemaRef {
        let mut new_fields: Vec<DataField> = Vec::new();
        for projection in projections.iter() {
            let field = self.find_field(projection).unwrap();
            new_fields.push(field.clone());
        }
        let new_schema = DataSchema::new(new_fields);
        return Arc::new(new_schema);
    }
}