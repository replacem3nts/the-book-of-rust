// In Rust, we separate tests into two types: 'Unit Tests,' and 'Integration Tests' (duh)

// Unit Tests: small + focused, testing one module at a time, can test private interfaces
// Integration Tests: entirely external to your library and use your code same as an external party
// would, potentially excercising multiple modules per test

// <-------------------------- Unit Test  -------------------------->
// The purpose is to test each unit of code in isolation to quickly  pinpoint where the
// code is not working

// They are located in the 'src' directory in each file with the code that they are testing
// The convention is to create a module named 'tests' in each file to contain the test
// functions and to annotate the module with cfg(test)

// <------------ The Tests Module and #[cfg(test)]
// >>> #[cfg(test)] <<< annotation tells Rust to only compile and run the test code when
// using the command 'cargo test' and not 'cargo build'
// This annotation is helpful for unit test since this code is located in the same file as
// the code it's testing

fn main() {
    println!("Hello, world!");
}
