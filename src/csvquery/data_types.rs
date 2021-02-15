use arrow;

pub type DataType = arrow::datatypes::DataType;

pub type DataField = arrow::datatypes::Field;

pub type DataSchemaRef = arrow::datatypes::SchemaRef;

pub type DataSchema = arrow::datatypes::Schema;

pub type DataArrayRef = arrow::array::ArrayRef;

#[derive(Debug, Clone)]
pub struct DataBlock {
    schema: DataSchemaRef,
    columns: Vec<DataArrayRef>,
}
