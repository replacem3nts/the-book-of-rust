fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Hello, {five}, {six}, {none}!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Code below will error since all cases are not
//  handled in the match block

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


// This code demonstrates use of a 'catch-all' arm where
// final pattern will match all if previous are not matched

// let dice_roll = 9;
// match dice_roll {
//   3 => add_fancy_hat(),
//   7 => remove_fancy_hat(),
//   other => move_player(other),
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}


// Below is same example as before but using the underscore char
// to catch all without using the value in the return block

// match dice_roll {
//   3 => add_fancy_hat(),
//   7 => remove_fancy_hat(),
//   _ => reroll(),
// }

// Finally, same example but 'run no code' instance

// match dice_roll {
//   3 => add_fancy_hat(),
//   7 => remove_fancy_hat(),
//   _ => (),
// }