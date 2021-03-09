use std::fmt::Display;

// generic type parameters, trait bounds, and lifetimes together

// because lifetimes are a type of generic, the declarations of the
// lifetime parameter 'a and the generic type parameter T go in
// the same list inside the angle brackets after the function name.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}
