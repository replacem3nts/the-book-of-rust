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

// <--------- Lifetimes Annotations in Struct Definitions --------->
// Structs can hold references but they require lifetime annotations when they are used

struct ImportantExcerpt<'a> {
    part: &'a str,
}
// The above annotation translates to: "an instance of this struct cannot outlive the
// reference it holds"

fn main() {
    let novel = String::from(
        "Call me Ishmael. Some years ago..."
    );
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find '.'");
// Creates new instance of struct, using ref to novel -->importantly!
// novel does not go out of scope until after ImportantExceprt
// goes out of scope, so the ref in the instance is valid
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// <-------------------- Lifetime Elision -------------------->
// While everything just discussed said that every reference has a lifetime and
// fns + structs with refs require specified lifetime params, we covered an example
// which compiled without that in an earlier chapter:

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Originally this would not have compiled, but eventually there were found to be
// a discreet number of common scenarios such that the compiler could identify them
// those cases are referred to as the "lifetime elision rules." They're instances where
// specifying lifetimes is not required because the compiler can infer the rule

// Lifetimes on function or method parameters are called input lifetimes,
// lifetimes on return values are called output lifetimes. --> compiler uses rules
// that exame input + output lifetimes to match cases

// RULES:
// 1. The compiler assigns a lifetime parameter to each parameter that’s a reference
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to
// all output lifetime parameters
// 3. If  there are multiple input lifetime parameters, but one of them is &self or &mut self
// because this is a method, the lifetime of self is assigned to all output lifetime parameters

// <-------------- Lifetimes in Method Definitions -------------->
// While we have already discussed that lifetime names for struct fields always need to be
// declared after the impl keyword, when annotating methods within the impl block, the
// situation is more complicated: references in the method signature could be tied to those
// of the struct fields or they could be independent

// <--------------------- Static Lifetimes --------------------->
// 'static denotes a reference can live for the duration of the program

let s: &'static str = "I have a static lifetime.";

// The text is stored directly in the programs binary (thus lifetime of all string literals
// is 'static)