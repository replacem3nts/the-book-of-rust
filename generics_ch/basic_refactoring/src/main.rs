fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list_one = vec![34, 10, 96, 26];

    let result_one = largest(&number_list_one);

    println!("The largest number in result one is: {result_one}");

    let number_list_two = vec![57, 29, 13, 7];

    let result_two = largest(&number_list_two);

    println!("The largest number in result two is: {result_two}");
  }

// If we wanted to duplicated the below functionality somewhere else,
// the below functionality could be abstracted into a fn that operates
// on any list of integers passed as a parameter (shown above)
// fn main() {
//     let number_list = vec![34, 10, 96, 26];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//       if number > largest {
//         largest = number;
//       }
//     }
//     println!("The largest number is: {largest}");
// }
