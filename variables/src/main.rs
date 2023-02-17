fn main() {
    let  x = plus_one(5);

    let x = x + 1;

    {
      let x = x * 2;
      println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
  x + 1
}