fn main() {
    let username = String::from("sk8tr_grl");
    let email = String::from("hello@trucks.edu");
    let user1 = build_user(username, email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
      active: true,
      username,
      email,
      sign_in_count: 1,
    }
}

struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}