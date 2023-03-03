fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual //This is a unit-like struct which does not have any fields at all