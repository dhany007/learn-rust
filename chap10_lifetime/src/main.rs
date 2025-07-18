fn main() {
    println!("Lifetime!");
}

use std::fmt::Display;

// 'a is lifetime
fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        X
    } else{
        y
    }
}