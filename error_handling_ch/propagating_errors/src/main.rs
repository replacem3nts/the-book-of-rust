// when implementation call something that may fail, instead of handling error within
// the function, can pass the error back to the calling code: "Propagation"
use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");
}

// return type of this function denotes type of Result<T, E> where T is String and E is io::Error
// io::Error is the chosen error type because its whats returned in the two possible failure locations
fn read_username_from_file() -> Result<String. io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        // if success: file handle becomes mutable var "username_file"
        Ok(file) => file,
        // if failure: "return" keyword for early return and pass error back to calling code
        Err(e) => return Err(e),
    };

    // this code only runs if early return doesn't run above
    let mut username = String::new();

    // logic below is the same as above except error block does not require explicit "return"
    // since this is the final expression of the function
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// The code that calls the function above will receive either an Ok value with a username
// or one of the two Err results prodced and can then decide what to do with them

// -----------------------------------------------------------------------------

// Pattern of propagating errors is so common that Rust provides a question mark operator for this purpose:
// fn read_username_from_file() => Result<String, io::Error> {
//     // -->> difference between the '?' (ln - 41) and match is errors go through the 'from' fn
//     // -->> (defined in the 'From' trait in the standard lib). When the '?' operator calls 'from' fn,
//     // -->> error type is converted into error type defined in current fn return type (ln - 36)
//     let mut username_file = File::open("hello.txt")?;

//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// -----------------------------------------------------------------------------

// Can be shortened further using the "?" operator by chaining method calls as shown below:
// fn read_username_from_file() => Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// -----------------------------------------------------------------------------

// Finally, the below is the shortest version of the initial fn:
// use std::fs;
// use std::io;
// fn read_username_from_file() => Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

//As you'd expect, reading a file into a string is common operation so std lib has a fn