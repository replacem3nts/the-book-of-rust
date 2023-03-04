fn main() {
  let some_number = Some(5);
  let some_char = Some('e');
  let absent_number: Option<i32> = None;

    println!("This is some_number: {:#?}", some_number);
    println!("This is some_char: {:#?}", some_char);
    println!("This is absent_number: {:#?}", absent_number);
}

// Code below will error out because 'sum' expression relies on y
// not being null. Rust forces you to use control flow to ensure
// null values are not used as if they were non-null.

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x + y;
//     println!("This is su: {}", sum);
// }