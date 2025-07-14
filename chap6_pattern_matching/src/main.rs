fn main() {
    println!("The Match Control Flow Construct!");
    // match that allows you to compare a value against a series of patterns
    // and then execute code based on which pattern matches.
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alabama));

    println!("Matching with Option<T>");
    let five = Some(5);
    let six = plus_one(five);
    println!("The value of six is {:?}", six);
    let none = plus_one(None);
    println!("The value of none is {:?}", none);
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

/*
    Let’s say we want to write a function that takes an Option<i32> and,
    if there’s a value inside, adds 1 to that value.
    If there isn’t a value inside, the function should return the None value
    and not attempt to perform any operations.
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

/*
    Using enums, we can also take special actions for a few particular values,
    but for all other values take one default action
    _ is a special pattern that matches any value and does not bind to that value
*/

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn match_dice_roll(dice: u8) {
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // we don’t want to run any code in this case.
    }
}