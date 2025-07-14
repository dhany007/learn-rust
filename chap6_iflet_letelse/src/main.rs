fn main() {
    println!("Control Flow with if-let and let-else");

    /*
        In other words, you can think of if let as syntax sugar for a match that runs code
        when the value matches one pattern and then ignores all other values.
    */
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum number is {}", max);
    }

    let state = describe_state_quarter(Coin::Quarter(UsState::Alabama));
    println!("{state:?}");
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// using let else
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1990) {
        Some(format!("{state:?} is pretty old, for america!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}