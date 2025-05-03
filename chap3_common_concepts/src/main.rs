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

    let mut name = "Jon";
    println!("name: {name}");
    // name = 10; // akan error karena data type tidak sama, expected `&str`, found integer
    // println!("name: {name}");
}
