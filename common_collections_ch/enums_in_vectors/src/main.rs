// vectors can only store one type of value, this can be
// more flexible if the one type of value is an enum which
// is ONE type but can consist of differing types
#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

fn main() {
    let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("Orange")),
      SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
      println!("The value of i is {:?}", i);
    }
} // <-- vector 'row' goes out of scope here
  // both vector and contents are dropped
