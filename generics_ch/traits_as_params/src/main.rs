// The below "notify" function requires an item as a paremeter, which
// implements the trait "Summary" --> tldr, we can specify a trait and
// the parameter has the needed methods accessible

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

// <------------------ Trait Bound Syntax ------------------->
// The above is actually syntactical sugar for longer form known as
// a... TRAIT BOUND (oooooooo!)

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

// While more verbose, this allows for more complicated definitions such
// as cases where you need multiple types.  Using the impl is required if
// the multiple items have different types, if they have the same type,
// they must use a "trait bound"

// <------------------ Multiple Trait Bound ------------------->
pub fn notify(item: &(impl Summary + Display)) {}
// =
pub fn notify<T: Summary + Display>(item: &T) {}
// (item will implement traits of both Summary and Display)

// <------------ Trait Bound with 'where' Clauses -------------->
// Cases with too many trait bounds can be difficult to read:
fn a_great_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// Using a 'where' clause can make things more readable:
fn a_great_function<T, U>(t: &T, u: &U) -> i32
where:
    T: Display + Clone,
    U: Clone + Debug
{}

// <-------- Returning Types That Implement Traits  ---------->
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// This is generally a handy feature when it comes to "closures" and "iterators"
// which are covered in detail in chapter thirteen.

// NOTE: there is a gotcha here, you cannot return two different types, even if
// both types implement the same trait. This is due to restrictions in how the
// compiler actually implements traits behind the scenes which will be covered later

// <----- Trait Bounds to Conditionally Implement Methods  ----->
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
} }
}
// In the above code, line 63 uses generic type parameters to implement a "new"
// function, then in line 69, that implementation is extended ONLY when T is of
// a type which implements the Display and the PartialOrd traits