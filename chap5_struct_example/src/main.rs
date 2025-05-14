fn main() {
    println!("An Example Program Using Structs");
    // a program that calculates the area of a rectangle.
    // We’ll start by using single variables,
    // and then refactor the program until we’re using structs instead.

    let width1 = 30;
    let height1 = 50;
    let result = area(width1, height1);
    println!("The area of the rectangle is {} square pixels", result);

    println!("Refactoring with Tuples");
    let rect1 = (30, 50);
    let result = area_tuples(rect1);
    println!("The area of the rectangle is {} square pixels", result);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}