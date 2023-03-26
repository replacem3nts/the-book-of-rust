// Code should panic when it could end up in a "bad state" (a term of art, apparently)
// "Bad States" are, for example, when an assumption, guarantee, contract or invariant
// has been broken: invalid values, contradictory values, missing values plus:
// 1. Something unexpected as opposed to something likly to happen occassionally
// 2. Code, after a designated point, relies on not being in this bad state vs checking at every step
// 3. There's no good way to cast info -> types (more later pg. 393 - "Encoding States and Behavior Types")

// ---------------------  Creating Custom Types for Validation  ---------------------

// Recalling the guessing game from earlier in the book, could instead use i32 vs. u32,
// (allow negative numbers) then validate the number range

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess must be between 1 and 100!, got {}",
                value
            );
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    println!("Hello, world!");
}
