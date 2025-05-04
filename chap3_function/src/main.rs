fn main() {
    println!("Function");
    // uses snake case as the conventional style for function and variable names,
    // in which all letters are lowercase and underscores separate words
    another_function(10);
    print_labeled_measurement(10, 'h');
}

fn another_function(x :i32){
    println!("The value of x is {x}");
}

fn print_labeled_measurement(x :i32, label :char) {
    println!("The value of x is: {x}{label}")
}