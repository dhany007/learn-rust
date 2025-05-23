fn main() {
    println!("Control Flow");

    println!("if condition");

    let number = 3;
    if number < 5  {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("if multiple condition");
    let number = 10;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("using if in let statement");
    let condition = true;
    let number = if condition {5} else {6};
    // let number = if condition {5} else {"6"}; // error because if and else not the same type
    println!("The value of number is: {number}");
}
