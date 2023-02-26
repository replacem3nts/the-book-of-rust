fn main() {
    let phrase = "Hello, world!";
    let w1 = first_word(&phrase);

    println!("the first word is: {w1}")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}