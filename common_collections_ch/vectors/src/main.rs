fn main() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(10);
    v1.push(9);
    v1.push(8);

    // This method will cause rust to panic, better for when
    // you want the program to crash if element not found
    let second: &i32 = &v1[2];
    println!("The second element of v1 is {second}.");

    let mut v2 = vec![1, 2, 3];

    v2.push(4);
    v2.push(5);

    // This method will not crash, since you are explicitly
    // handling the error case with the match control flow block
    let fifth: Option<&i32> = v2.get(4);
    match fifth {
      Some(fifth) => println!("The fifth element of v2 is {fifth}."),
      None => println!("There is no fifth element."),
    }
}

// The below code will not compile because vectors put values next to
// each other in memory. Adding an element could change the required
// memory allocation such the vector be moved to new space. This would
// leave the immutable borrow pointing to memory that no longer exists.

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     // immutable borrow here
//     let first = &v[0];

//     // mutable borrow here
//     v.push(6);

//     // immutable borrow used here as well.
//     println!("The fifth element of v2 is {first}.");
// }
