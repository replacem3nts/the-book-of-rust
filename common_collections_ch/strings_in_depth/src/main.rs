// string slices are **references** to some UTF-8 encoded string data
// the String type provided by the Rust standard library is a growable
// mutable, owned UTF-8 encoded string type

fn main() {
    // String is actually implemented as a wrapper around a vector of bytes, so it
    // offers many of the same operations as with Vec<T>
    let mut s = String::new();

    // If we have some initial data we want to start a string we can use the "to_string" method

    let data = "initial contents";
    s = data.to_string();

    // The method also works on a literal directly:
    // let s = "initial contents".to_string();

    // Yet a third way is using the namespaced "from"
    // let s = String::from("initial contents");

    s.push_str(" --> plus a little more");
    s.push('!');
    println!("Hello, world: {s}");
}

// strings are UTF-8 encoded so any properly encoded data can be included:
// let hello = String::from("السلام عليكم");
// let hello = String::from("Dobrý den");
// let hello = String::from("Hello");
// let hello = String::from("שָׁלוֹם");
// let hello = String::from("नमस्ते");
// let hello = String::from("こんにちは");
// let hello = String::from("안녕하세요");
// let hello = String::from("你好");
// let hello = String::from("Olá");
// let hello = String::from("Здравствуйте");
// let hello = String::from("Hola");