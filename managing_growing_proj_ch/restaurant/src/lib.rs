// When using multiple multiple parts of the same path,
// that can be done as seen below
use std::{cmp::Ordering, io};

// Also possible to import all public items within a scope
use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
      fn take_order() {}

      fn serve_order() {}

      fn take_payment() {}
    }
}

//Declaring with 'use' keyword for reuse
use crate::front_of_house::hosting;

//Declaring with 'use' & 'pub' keyword for re-exporting
pub use crate::front_of_house::hosting;

// Declaring with 'use' and aliasing
use crate::front_of_house::serving as ToServe;

pub fn eat_at_restaurant() {
  // Reuse with 'use' keyword
  hosting::add_to_waitlist();

  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();

  // Reuse with 'use' plus alias
  ToServe.take_order();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Using super to reference anything from the root module
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit:String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Unlike structs, all enum variants are public by default
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_breakfast() {
    // Order a breakfast in summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Switch the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Next line won't compile since not public
    meal.seasonal_fruit = String::from("blueberries");
}