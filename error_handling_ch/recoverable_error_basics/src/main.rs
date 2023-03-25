use std::fs::File;
use std::io::ErrorKind;


fn main() {
    // when File:open succeeds it will return an instance of `Ok` that contains a file handle
    // when it fails, it will return an instance of `Err` that contians information on the kind of error
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // possible to use flow control on different types of errors by using an inner match fn
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Problem creating the file: {:?}",
                        e
                    ),
                }
            }
            other_error => {
              panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// Above is a lot of match logic, below is an alternative
// This uses closures which are covered more deeply in chapter 13

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}, error");
//         }
//     });
// }

// Result<T, E> has many helper helper methods such as "unwrap" and "expect" below:
// NOTE: Per tBoR "most Rustaceans choose 'expect' rather than 'unwrap' to provide more context"

// UNWRAP:
// on 'Ok' variant will return value inside 'Ok'
// on 'Err' variant will call 'panic!' macro
// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap();
// }
// NO FILE: thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
// Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:52:49
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//EXPECT:
// same as unwrap but takes as an argument an message to display when 'panic!' macro fires
// fn main() {
//       let greeting_file = File::open("hello.txt")
//           .expect("hello.txt should be included in this project");
// }