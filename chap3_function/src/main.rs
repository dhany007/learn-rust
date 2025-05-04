fn main() {
    println!("Function");
    // uses snake case as the conventional style for function and variable names,
    // in which all letters are lowercase and underscores separate words
    another_function(10);
    print_labeled_measurement(10, 'h');

    println!("Statements and Expressions");
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.
    let y = {
        let x = 5;
        x + 1
    };
    println!("The value of y is: {y}"); // y = 6

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(10);
    println!("The value of y is: {y}");

    let z = plus_two(20);
    println!("The value of z is {z}");
}

fn another_function(x :i32){
    println!("The value of x is {x}");
}

fn print_labeled_measurement(x :i32, label :char) {
    println!("The value of x is: {x}{label}")
}

fn five() -> i32 {
    5 // do not use semicolon here because we not use return
}

fn plus_one(x :i32) -> i32 {
    x + 1 // remove this semicolon to return this value
}
