use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // let apples = 5; // immutable by default
    // println!("apples {}", apples);
    // // apples = 10; // error because immutable
    //
    // let mut bananas = 10; // mutable
    // println!("bananas {}", bananas);
    // bananas = 20;
    // println!("new bananas {}", bananas);

    // let x = 10;
    // let y = 20;
    // println!("x = {x} and y + 2 = {}", y+2);

    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    // rng = random number generator we’re going to use
    // random_range = generates a random number in the range

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // create new mutable variable and bound to new empty string

        io::stdin() // obtain user input
            .read_line(&mut guess) // append to empty string, use mutable pass by reference (&mut)
            .expect("Failed to read line"); // handling potential error

        let guess: u32 = match guess.trim().parse() { //using match for parse
            Ok(num) => num, // enum ok will return number
            Err(_) => continue, // enum error will continue the loop
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){ // cmp= compare, will return Less, Greater, Equal
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("Thank you")
}
