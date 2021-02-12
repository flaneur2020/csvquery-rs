use super::data_row::DataRow;

#[derive(Debug)]
pub struct DataSet<'a> {
    rows: Vec<DataRow<'a>>,
}

impl<'a> DataSet<'a> {
    pub fn empty() -> Self {
        let rows: Vec<DataRow<'a>> = vec![];
        Self {
            rows: rows,
        }
    }
}