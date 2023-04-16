// Bodies of test functions typically perform three actions:
// 1. Set up any needed data or state
// 2. Run the code that you want to test
// 3. Assert that the results are what you expect

// <----------------- Anatomy of a Test Function ----------------->
// Simple def: A Rust function annotated with a test attribute
// Adding "#[test]" on the line before a fn creates a test
// When you run "cargo test" Rust creates a test runner binary and reports pass/fail

//OUTPUT OF NEW LIB TEST RUN:
// $ cargo test

// Compiling adder v0.1.0 (file:///projects/adder)
//       Finished test [unoptimized + debuginfo] target(s) in 0.57s
//        Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test  <--CAN see the one test that's defined is running
// test tests::it_works ... ok  <--SHOWS the name of the generated test function, result is "OK"
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0  <--OVERALL result summary is "OK,"
//                                                                <<<NOTE (^)>>> it's possible to mark
//                                                                tests as ignored, more on that later
// filtered out; finished in 0.00s
//
//     Doc-tests adder  <--RESULTS of any documentation tests (Rust can compile code
//                      examples that appear in our documentation)
//
//   running 0 tests
//
//   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0
//   filtered out; finished in 0.00s

//OUTPUT OF INTENTIONAL FAILURE TEST RUN:
// running 2 tests
// test tests::exploration ... ok
// test tests::another ... FAILED  <---SHOWS "FAILED" instead of "ok"
//
// failures:
//
// ---- tests::another stdout ----  <---SECTION displays the detailed reason that the test failed
// thread 'main' panicked at 'Make this test fail', src/lib.rs:10:9
// note: run with `RUST_BACKTRACE=1` environment variable to display
// a backtrace
//
// failures: <---LISTS names of failing tests (useful when lots!)
//      tests::another
//
// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0  <--DISPLAYS summary info
// filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// <---------- Checking Results with the assert! Macro ---------->

fn main() {
    println!("Hello, world!");
}
