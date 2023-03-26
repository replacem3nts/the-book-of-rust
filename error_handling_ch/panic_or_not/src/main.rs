// As a general rule, returning Result is a good default, panic! is good for situations
// like examples, prototyping, and running tests

// ---------------------  Examples, Prototype Code, and Tests  ---------------------
// 1. It's understood that calling "unwrap" (could panic!) is a placeholder for error handling
// 2. They're also helpful before you've decided how to handle errors since they leave markers
// 3. In testing, if a method call fails, you'd want to immediately fail the whole test

// ----------------  When You're Better Informed Than the Compiler  ----------------
// If you have logic elsewhere to ensure an "Ok" value will exist, the compiler is not going
// to be privy to that information. In this case, calling "unwrap" is acceptable, and its good
// practice to document why you should never have an Err variant

use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("Hello, world!");
}
