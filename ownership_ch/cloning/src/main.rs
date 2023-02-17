fn main() {

    let s1 = String::from("hello");

    let mut s2 = s1.clone();

    s2.push_str(" darth");


    println!("This is the value of s1: {s1}");

    println!("This is the value of s2: {s2}");
}