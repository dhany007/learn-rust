fn main() {
    println!("An Example Program Using Structs");
    // a program that calculates the area of a rectangle.
    // We’ll start by using single variables,
    // and then refactor the program until we’re using structs instead.

    let width1 = 30;
    let height1 = 50;
    let result = area(width1, height1);
    println!("The area of the rectangle is {} square pixels", result)
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}