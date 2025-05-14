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

    println!("Refactoring with Structs: Adding More Meaning");
    let rect2 = Rectangle{
        width: 30,
        height: 50,
    };
    let result = area_structs(&rect2);
    println!("The area of the rectangle is {} square pixels", result);

    println!("Adding Useful Functionality with Derived Traits");
    // use {:#?} for formatting print
    // use #[derive(Debug)] in struct for debug
    println!("rect2 is {rect2:#?}");

    //  dbg!, which takes ownership of an expression (as opposed to println!, which takes a reference),
    // prints the file and line number of where that dbg! macro call occurs in your code
    // along with the resultant value of that expression, and returns ownership of the value.

    let scale = 2;
    let rect4 = Rectangle{
        width: dbg!(30 * scale),
        height: 50
    };
    dbg!(&rect4); // We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call
    // Note: Calling the dbg! macro prints to the standard error console stream (stderr),
    // as opposed to println!, which prints to the standard output console stream (stdout)

    // The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
