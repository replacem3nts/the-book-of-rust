// Hash maps are least commonly used so they are not automatically
// included in scope and must be imported
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    // Hash maps are homogeneous, all keys/values must have the same types
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // "get" method returns an Option<&V> or none
    // below returns an Option<i32> rather than a borrowed value by using the "copied" fn
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("This is the score: {score}!");

    // Hash maps can be iterated over similar to vectors with a for loop
    for (key, value) in &scores {
      println!("The value of {key} is {value}");
    }
}

// --->> OWNERSHIP <<---
// Types that implement copy (like  i32) will have values copied into hash maps
// Owned values like Strings will cede ownership to the hash map

// use std::collections::HashMap;
// let field_name = String::from("Favorite color");
// let field_value = String::from("Blue");
// let mut map = HashMap::new();
// map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try
// using them and see what compiler error you get!
