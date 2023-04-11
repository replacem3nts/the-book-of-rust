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
fn main() {
    let str1 = String::from("wxyz");
    let str2 = "abc";

    let result = longest(str1.as_str(), str2);
    println!("The longest string is: {result}")
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Above code will throw an error saying: expected return value to be a "named lifetime
// parameter. help: does not say if returned borrowed value is x or y"

// Since neither we, nor the borrow checker can determine the answer of which will be
// returned, we add 'generic lifetime parameters' that define the relationship between
// the references so borrow checker can analyze

// <----------------- Lifetime Annotation Syntax  ----------------->
// >> Lifetime annotations don't change how long lifetimes live. They describe the
// relationships of multiple references to each other without affecting the lifetimes. <<

// Syntax: always starts with an apostrophe ('); usually always lowercase and very short
// comes after the & but before the reference type as seen below:

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// <----------------- Lifetime in Fn Signatures  ----------------->
// Syntax similar to generic type parameters in fns: declared inside angle brackets
// between fn name and param list as seen below:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//This notation signifies: "the returned reference will be valid as long as both params
// are valid"

// Lifetime annotations always go in the function signature, never in the function body.
// Giving fn signatures lifetime contracts means Rust compiler analysis is simplified

// In the above example, when the function receives concrete references, 'a will take on
// the concrete lifetime which is the smaller of lifetimes x and y.

// <--------------- Thinking in Terms of Lifetimes --------------->
//