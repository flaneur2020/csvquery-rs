mod csvquery;

use csvquery::CSVQueryExecutor;

fn main() {
    let sql = "abc";
    let executor = CSVQueryExecutor::new(sql);
    let result = executor.execute();
    println!("result: {:?}", result.unwrap());
}
