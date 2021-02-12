use crate::csvquery::data_cell::DataCell;

#[derive(Debug)]
pub struct DataRow<'a> {
    columns: Vec<DataCell<'a>>,
}