// In Rust, we separate tests into two types: 'Unit Tests,' and 'Integration Tests' (duh)

// Unit Tests: small + focused, testing one module at a time, can test private interfaces
// Integration Tests: entirely external to your library and use your code same as an external party
// would, potentially excercising multiple modules per test

// <-------------------------- Unit Tests  -------------------------->
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

// <------------ Testing Private Functions
// There's a debate whether or not private functions should be tested directly...
// Other languages take an opinionated stance (preventative), Rust allows it

// Since tests are just regular Rust code, you can bring them into scope and test them as
// you would anything else--if you're on the "anti"-side of the debate, you are not compelled
// to do this

// <-------------------------- Integration Tests  -------------------------->
// As previously stated, integration tests work the same as if a third party were using your code
// The purpose is to make sure that the many parts of a library work together correctly--in other
// words, things that work fine on their own can cause issues when they are combined

// To create integration tests, you first need a 'tests' directory:
// [A directory called 'tests' was created in the 'adder' example project folder. the 'tests'
// folder is parallel to the src folder and contains a file named 'integration_test.rs']

//    Compiling adder v0.1.0 (/Users/northernspies/Development/the-book-of-rust/writing_tests_ch/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.87s
//      Running unittests src/lib.rs (/Users/northernspies/Development/the-book-of-rust/writing_tests_ch/adder/target/debug/deps/adder-a47970e56575f063)

// running 1 test
// test tests::it_adds_two ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//      Running tests/integration_test.rs (/Users/northernspies/Development/the-book-of-rust/writing_tests_ch/adder/target/debug/deps/integration_test-bf7ca22e7a16a552)
//
// running 1 test
// test it_adds_two ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
