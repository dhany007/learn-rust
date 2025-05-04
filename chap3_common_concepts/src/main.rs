use std::io;

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

    println!("Constant");
    // same as immutable
    // Rust’s naming convention for constants is to use all uppercase with underscores between words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
    const PHI: f32 = 3.14;
    println!("PHI: {PHI}");

    println!("Shadowing");
    let x = 5;
    let x = x + 1; // x = 6
    {
        let x = x * 2; // x = 12
        println!("The value of x in inner scope is: {x}") // x = 12
    }
    println!("The value of x is: {x}"); // x = 6, because outer scope


    // if not used `mut`, maka tipe data tidak dipermasalahkan
    let spaces = "   ";
    println!("spaces: {spaces}");
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // let mut name = "Jon";
    // println!("name: {name}");
    // name = 10; // akan error karena data type tidak sama, expected `&str`, found integer
    // println!("name: {name}");

    println!("Data Types");
    // must know the types of all variables at compile time
    // let guess = "42".parse().expect("Not a number!"); // error: consider giving `guess` an explicit type

    let guess :u32 = "42".parse().expect("Not a number!");
    println!("Number: {guess}");

    // two data type subsets: scalar and compound
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

    let num = 10; // default i32
    let num :u64 = 10_000;
    let point = 10.0; // default f64
    let y : f32 = 10.1;

    let sum = 5 + 10; // addition
    let difference = 10.9 - 9.8; // substraction
    let product = 4 * 20; // multiplication
    let quotient = 10.8 / 8.1; // division
    let truncated = -5/3;
    let reminder = 10 % 3; // modulo

    let t = true;
    let f: bool = false; // explicit

    let c = 'z';
    let z :char = 'Z'; // explicit


    println!("Compound");
    // Compound types can group multiple values into one type.
    println!("The Tuple Types");
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type

    let tup :(i32, f32, u8) = (100, 5.3, 10);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let hundred = tup.0;
    let five_three = tup.1;
    let ten = tup.2;
    println!("the of hundred is : {hundred}");
    println!("the of five_three is : {five_three}");
    println!("the of ten is : {ten}");

    println!("The Array Types");
    // every element of an array must have the same type
    let order = [1,2,3,4,5];
    let days = ["Monday", "Tuesday", "Wednesday"];

    // initialize array
    let x :[i32; 5] = [1,2,3,4,5];
    let y = [3;5]; // [3, 3, 3, 3, 3]
    let first = x[0];
    println!("first is: {first}");

    let a = [1,2,3,4,5];
    println!("please enter an array index (separate by space): ");
    let mut arr = String::new(); // initialize empty string

    io::stdin()
        .read_line(&mut arr)
        .expect("Failed to read line");

    let index: usize = arr
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = a[index];
    println!("The value of element at index {index} is : {element}");
}
