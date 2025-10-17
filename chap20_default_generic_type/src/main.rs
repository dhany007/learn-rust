use std::ops::Add;

fn main() {
    println!("Default Generic Type Parameters");

    assert_eq!(
        Point {x:1, y:0} + Point {x:2, y:3}, Point {x: 3, y:3}
    )
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Rust memakai default Add<Point>
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}