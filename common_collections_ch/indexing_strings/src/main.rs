// Unlike other languages, Rust does not permit accessing of individual chars
// by using indexing syntax. The below code errors out:
// fn main() {
//     let s = String::from("Beetlejuice")
//     let j = s[7];
// }


// Under the hood, a String is a wrapper over a Vec<u8>, so strings are
// encoded as bytes and chars of differing lengths can have different numbers
// of bytes which may not be fully expected:
// "hola" = 4 "chars" -> 4 bytes
// "Здравствуйте" = 12 "chars" -> 24 bytes


// From Rust's perspective, UTF-8 can be 1. Bytes, 2. Scalar Values,
// or 3. Grapheme Clusters (the closest thing to what we would call letters)

// While you can slice strings, the syntax is performed on the value in bytes
// and if a byte is chosen which is only part of a full character, Rust will crash

// fn main() {
//     let hello = "Здравствуйте"; // each "char" is two bytes
//     let s = &hello[0..1]; // this will error since 1 is only half of "З"
// }

// To access parts of String the recommended method is by iterating and explicitly
// defining whether characters or bytes are being used as shown below.

fn main() {
    let hello = "Зд";

    for c in hello.chars() {
      println!("{c}");
    }
    // Prints:
    // З
    // д

    for b in hello.bytes() {
      println!("{b}");
    }
    // Prints:
    // 208
    // 151
    // 208
    // 180
}
