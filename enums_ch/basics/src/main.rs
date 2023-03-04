#[derive(Debug)]
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);
}

fn route(ip_kind: IpAddrKind) {
  println!("This is an enum: {:#?}.", ip_kind);

}