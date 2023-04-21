use adder; // <-- each file in the tests dir is a separate crate so need to bring
            // things into scope to test them...

// no need to annotate with #[cfg(test)]--Cargo treats the dir specially
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}