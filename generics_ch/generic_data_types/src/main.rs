// <-------------------- Performance Characteristics of Generics -------------------->
// Generic types will not cause programs to run any slower than they otherwise would
// when using concrete types (woohoo!)

// Under the hood, when Rust compiles, takes all instances of generics and converts
// them into distinct concrete types. So a generic:

enum Thing<T> {
    x: T,
}

// with an i32 version and an f64 version, at compilation will become something like:

enum Thing_i32 {
    x: i32,
}

enum Thing_f64 {
    x: f64,
}

// <-------------------- Method Defintions -------------------->
// Also possible to use different types in a struct definition and the method
// signatures of that same struct:
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: Point<X2, Y2>,
    ) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


// In contrast to the example below, we could choose to constrain the generic type for
// which the function can be used:

impl Point<i32> {
    fn distance_from_origin(&self) -> i32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Below code takes the earlier example with a coordinate struct and implements a "getter"
// method that makes use of the generic typing.
// NOTE: By declaring the generic type after "impl" Rust knows that the type inside the Point
// T is a generic type and NOT a concrete type
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}


// <-------------------- Enum Defintions -------------------->
// Very straightforward usecases that have already been covered such as the "Option"
// as well the "Result" as shown below:
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// <-------------------- Struct Defintions -------------------->
// Since the below code does not restrict the possible types for "T",
// all it really says is that x and y are of the same type:
// (x: i32, y: u16 will not compile)
struct Point<T> {
    x: T,
    y: T
}

// The below allows but does not require that x+y are different types
struct Point<T, U> {
    x: T,
    y: U
}

// <-------------------- Generics Basics -------------------->
// In addition to abstracting the type into a generic, the below function
// makes one additional req change: the method performed in the body is not
// valid for all possible values of "T", so the standard library provides
// a means for restricting the values of "T" to types that can be compared
// (the "cmp" refers to comparison)
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
        largest = item;
      }
  }

  largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}

// Two functions below perform the same action on two different data types
// They can be combined into one using generics as shown above
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }