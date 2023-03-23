// number of key/value pairs is growable but keys:values -> 1:1
// because of this, must provide for the case where a key already has a value assigned

// Overwriting an existing value:
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
    // prints "{Blue: 25}" since value was overwritten

    // "entry" is a special API that allows to check for existing key in hash map
    scores.entry(String::from("Yellow")).or_insert(50);
    // "or_insert" method returns a mutable reference if it exists or inserts param
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    // Yellow now exists, but Blue remains as is, since it fails the "entry" function


    let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map);
}
