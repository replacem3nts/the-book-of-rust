// "?" can only be used in fns with return type compatible with the value ? is used on
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
// ERROR: the `?` operator can only be used in a function that returns `Result`
// or `Option` (or another type that implements `FromResidual`)

// To fix this, can 1. Change return type, or 2. use a 'match' / Result<T, E> method

// -----------------------------------------------------------------------------

// Separately, the "?" can also be used on fns that return an Option<T> (result is None or Some)
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

// In the one liner above, when "lines" iterates over the borrowed string slice,  "next" may find
// a line of text and return "Some" or it may not and return "None"

// -----------------------------------------------------------------------------

// NOTE: "?" operator can be used on a Result fn or an Option fn, but you can't mix and match