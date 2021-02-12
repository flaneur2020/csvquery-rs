#[derive(Debug)]
pub enum DataCell<'a> {
    Int(i64),
    Float(f64),
    String(&'a str),
}