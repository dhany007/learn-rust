fn main() {
    println!("Common Programming concept");

    println!("Variable and Mutability");
    let x = 5;
    println!("The value of x is: {x}");
    // x = 10; // error because immutable
    // println!("The value of x is: {x}");

    let mut y = 10; // mutable
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");
}
