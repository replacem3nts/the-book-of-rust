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

fn main() {
    println!("Hello, world!");
}
