fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // function returns a reference to a string
    let s = String::from("hello"); // the new string

    &s // returning the reference to s
} // here, s goes out of scope so its memory goes away--problemo!