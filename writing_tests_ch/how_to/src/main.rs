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
// [see lines 39 - 51 in the adder directory]
// Pretty much is what it says it is, 'assert!' accepts an expression which passes
// when it evalutes to true and fails when it evaluates to false

// <---------- Testing equality with assert_eq! and assert_ne! ---------->
// Providing an expected value to test that a function produces it correctly is so
// common that the standard library provides two out of the box macros for that ver
// purpose.

// assert_eq! - compares two arguments for equality
// assert_ne! - compares two arguments for inequality
// will also print the two values to make it easier to see why it failed

// under the surface, the macros use the '==' and the '!=' operators respectively

// <---------------- Adding Custom Failure Messages ---------------->
// With all of the previously discussed macros, arguments specified after the
// required arguments will be passed to the format! macro
// See example below:

#[test]
fn greeting_contains_name() {
  let result = greeting("Carol");
  assert!(
    result.contains("Carol"),
    "Greeting did not contain name, value was `{result}`"
  );
}

// <---------------- Checking for Panics with should_panic ---------------->
// Testing can also be used to check that our code is erroring when it should be
// This can be done using the attribute 'should_panic': if code inside the function
// panics, it passes (and visa versa)

// #[test]
// #[should_panic] <--THING that does what we just talked about
// fn greater_than_100() {
//     Guess::new(200);
// }

// There is a weakness to 'should_panic' which is that a fn which panics for a reason
// other than the one which we intended, will still pass. To fix this, we can add an
// optional 'expected' attribute which will check that the failure message contains
// the provided text:

// #[test]
// #[should_panic(expected = "less than or equal to 100")] <--PASSING failure text
// fn greater_than_100() {
//     Guess::new(200);
// }

// <---------------- Using Result<T, E> in Tests ---------------->
// So far all tests have failed by panicking--alternatively, tests can be written
// that use Result<T, E>, ex below:


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

// With Result<T, E> you can use the question mark operator in test bodies which allows
// you to write tests that will fail if any operation within them returns an Err variant
// NOTE: you can't use the '#[should_panic]' notation with these type of tests