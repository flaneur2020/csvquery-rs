use std::sync::Arc;

pub type DataArrayRef = Arc<DataArray>;

#[derive(Debug)]
pub enum DataArray {
    IntArray(Vec<i64>),
    StringArray(Vec<String>),
    FloatArray(Vec<f64>),
}
