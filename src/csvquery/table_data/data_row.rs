use super::data_cell::DataCell;

#[derive(Debug)]
pub struct DataRow<'a> {
    columns: Vec<DataCell<'a>>,
}