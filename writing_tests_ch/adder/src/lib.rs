pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // indicates this is a test function
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); // this is a macro which is used to assert the result
    }

    #[test]
    fn another() {
        panic!("Make this test fail!")
    }
}
