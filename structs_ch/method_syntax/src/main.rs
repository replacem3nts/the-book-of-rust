// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//           self.width * self.height
//     }
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     println!(
//       "The area of the rectangle is {} square pixels.",
//       rect1.area());
// }


struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
    }
}

// It is valid to have multiple impl blocks and there can be cases where it makes sense
impl Rectangle {
      // Example of an associated function
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Associated functions are invoked by namespacing ::
    let sq = Rectangle::square(8);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("The area of the square is {}", sq.area());
}