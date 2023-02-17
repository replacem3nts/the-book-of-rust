fn main() {
  let mut s = String::from("hello");

    {                   // s is not valid here sit it's not yet declared
       let _s = "goodbye"; // s is valid from this point forward
                        // do stuff with s
    }                   // this scope is now over, s is no longer valid

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}");
}