#[derive(Debug)]
enum UsState {
  Alaska,
  Alabama,
  Arizona,
  Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin1 = Coin::Quarter(UsState::Arkansas);
    let value: u8 = value_in_cents(coin1);
    println!("The value is: {}", value);
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {    // Multi-line return blocks req curly braces
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:#?}!", state);
      25
    }
  }
}
