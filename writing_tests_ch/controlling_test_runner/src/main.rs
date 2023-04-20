// Just like 'cargo run' the 'cargo test' command compiles the code into a
// binary, the difference is the compilation is done in 'test mode'

// The default behavior runs all tests in parallel and capture output generated
// during test runs to prevent output being displayed and making it easier to read

// There are CLI options to change this default behavior (cargo test --help)

// <----------- Running Tests in Parallel or Consecutively  ----------->
// Default parallel behavior is faster, but requires that none of the tests depend
// on each other or any shared state.

// Example:
// Say each of your tests runs some code that each creates a file output named
// "test-output.txt" Then each test reads the data in that file and asserts
// that the file contains a particular value (different for each test case)

// Since one test may overwrite the file in the time between writing + reading
// from a different test, the test may fail erroneously

// One solution to this is to run each test consecutively

// ---------------------------------
// $ cargo test -- --test-threads=1
// ---------------------------------

// Single-threaded runtime = 0 parrelellism

// <-------------------- Showing Function Output  -------------------->
// By default, -->>IF A TEST PASSES<<-- (fails print), Rust captures anything printed to
// standard output (if there's a 'println!' fn, we won't see any output in the terminal)

// We can override this behavior with the command:

// ---------------------------------
// $ cargo test -- --show-output
// ---------------------------------

// <--------------- Running a Subset of Tests by Name  --------------->
// Test names can be passed to the 'cargo test' command to achieve ^

// To run a single test, pass the name of the test function to 'cargo test':

// ---------------------------------
// $ cargo test add_one_hundred
// ---------------------------------

// Cannot specify multiple names, will only read the first value
// To run multiple tests: you can specify part of a name. For instance,
// if we had a number of text functions with 'add' contained in the name,
// the below command would run each of those

// ---------------------------------
// $ cargo test add
// ---------------------------------

// <------ Ignoring Some Tests Unless Specifically Requested  ------>
// It is possible to add an attribute to exclude it as shown in the example below:


// #[test]
// #[ignore]
// fn expensive_test() {
//     // code that takes an hour to run
// }

// This prevents you from having to cherry pick around expensive tests
// Then, in cases where we would like those expensive tests included, we can use:

// ---------------------------------
// $ cargo test -- --ignored
// ---------------------------------

