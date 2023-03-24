// The below, attempting to access an index beyond the end of an array
// is explicitly called out as an error that will trigger a panic
fn main() {
    let v = vec![1, 2, 3];

    // This behavior (of aborting the program when attempting to access
    // an index beyond) is actually a security feature--in C, this can
    // return undefined, but it can also return whatever value is in memory
    // in that location and can be manipulated in certain cases
    v[99];
}

// Most basic example to purposely call a panic error
// fn main() {
//     panic!("crash y burn!");
// }
