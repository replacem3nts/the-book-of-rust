fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
      println!("{i}");
    }

    let mut v1 = vec![38, 99, 17, 29];
    for i in &mut v1 {
      *i += 50;
      println!("{i}");
    }
}
