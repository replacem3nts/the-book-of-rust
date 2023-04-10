// Lifetimes ensure that references are valid as long as they're needed

// Every reference in Rust has a lifetime, they must only be explicitly
// annotated when "lifetimes of references could be related in a few ways"

// <----------------- Preventing Dangling References  ----------------->
// Primary aim of lifetimes is to prevent ^, where programs reference data
// other than the data its supposed to reference

fn main() {
    let r;
    {
      let x = 5;
      r = &x;
    }

    println!("Lets print r: {}", r);
}

// The above code will error saying, "[x] borrowed value does not live long enough"
// "r" is still a valid reference but it "lives longer" than "x" does, if it ran it
//  would be referencing memory that went out of reference when "x" went out of scope.
// Rust makes this determination using a "borrow checker"

// <------------------------ Borrow Checker  ------------------------>
// Rust compiler has a ^ that compares scopes to ensure that all borrows are valid
// They show some examples, but the sentence above pretty much captures it

// <---------------- Generic Lifetimes in Functions  ---------------->
//