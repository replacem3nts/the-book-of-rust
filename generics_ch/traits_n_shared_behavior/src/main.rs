// <-------------------- Defining a Trait -------------------->
// Traits create groups of method signatures that can be used to provide
// common functionality sets for different types. The important part of
// the above sentence is a trait ONLY specifies a method signature, the
// method implementation is specified by the type that possesses the trait.
// In other words, in the below example, a type with the "Summary" trait
// must also provide a method "summarize" which takes the same inputs and
// provides the same outputs.

pub trait Summary {
    // End of line below is a semi-colon since this is just a signature
    // implementation must be provided by the type which has Summary trait
    fn summarize(&self) -> String;
}


// <-------------- Implementing Trait on a Type ------------->
// Implementing trait on type is similar to implementing regular methods,
// impl [trait-name] for [type] {}. Then within the impl block, define the
// methods which correspond to the method signatures on the trait
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Once implemented on a type, users of a crate can call trait methods as they
// would regular methods
// NOTE: only difference is that trait MUST be brought into scope (below) the
// same way that the type itself is brought into scope

use aggregator::{Summary, Tweet};

fn main() {
    let tweet = new Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "Mr. Ed, like many a noble equine, was foaled in an apartment in the Bronx"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// Important caveat: either the trait, the type, or both must be local to the crate to
// be able to implement a trait on a type--implementing an external trait on an external
// type is not allowed (this property is called "coherence" aka "the orphan rule")
// Reasoning for ^ : without this, two crates could implement the same trait and Rust
// wouldn't know which to use

// <----------------- Default Implementations ----------------->
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle{}

// Seen above, specifying a default implementation for a trait is as simple as providing
// the {} implementation block in the trait

// Default implementations can also call other methods within the trait, even if they
// themselves do not have their own default implementation

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more from: {})",
        self.summarize_author()
      )

    }
}

// NOTE: It is not possible to call a **default implementation** from an overriding method
// implementation of that same method