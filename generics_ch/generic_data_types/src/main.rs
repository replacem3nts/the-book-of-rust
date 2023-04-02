
// <-------------------- Method Defintions -------------------->
// <-------------------- Enum Defintions -------------------->


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