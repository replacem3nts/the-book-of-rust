// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test] // indicates this is a test function
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4); // this is a macro which is used to assert the result
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail!")
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//   width: u32,
//   height: u32,
// }
// impl Rectangle {
//   fn can_hold(&self, other: &Rectangle) -> bool {
//     self.width > other.width && self.height > other.height
//     // self.width < other.width && self.height < other.height <---INTRO'D BUG TO DEMONSTRATE FAIL
//   }
// }

// #[cfg(test)]
// mod tests {
//     use super::*; //this is a regular module that follows usual visibility rules, so need
//                   // to bring the code under test in the outer module into the inner module

//     #[test]
//     fn larger_can_hold_smaller() { // name of the test
//       let larger = Rectangle { width: 8, height: 7, }; // first of two rectangles
//       let smaller = Rectangle { width: 5, height: 1, };
//       assert!(larger.can_hold(&smaller)); // calling assert to check result of can_hold fn
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//       let larger = Rectangle { width: 8, height: 7, };
//       let smaller = Rectangle { width: 5, height: 1, };
//       assert!(!smaller.can_hold(&larger));
//     }
// }


pub fn add_two(a: i32) -> i32 { a+ 2
}
  #[cfg(test)]
  mod tests {
      use super::*;
      #[test]
      fn it_adds_two() {
          assert_eq!(4, add_two(2));
      }
}