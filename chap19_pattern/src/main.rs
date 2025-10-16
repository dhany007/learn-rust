fn main() {
    println!("match Arms!");
    /*
        semuanya harus didaftarkan atau menggunakan _
    */

    let dir = Direction::Up;

    match dir {
        Direction::Down  => println!("Down content"),
        Direction::Up  => println!("Up content"),
        Direction::Left  => println!("Left content"),
        Direction::Right => println!("Right content"),
    }

    // or
    match dir {
        Direction::Down => println!("Down content"),
        _ => println!("Other content")
    }

    // Conditional if let Expressions
    println!("Conditional if let Expressions");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // transform to match
    match favorite_color {
        Some(color) => println!("Using your favorite color, {color}, as the background"),
        None => {
            match is_tuesday {
                true => println!("Tuesday is green day!"),
                false => match age {
                    Ok(age) if age > 30 => {
                        println!("Using purple as the background color")
                    },
                    Ok(_) => {
                        println!("Using orange as the background color")
                    },
                    Err(_) => {
                        println!("Using blue as the background color")
                    }
                }
            }
        }
    }

    // while let Conditional Loops
    println!("while let Conditional Loops");
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1,2,3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}")
    }

    // for Loops
    println!("for Loops");
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // let Statements
    println!("let Statements");
    let x = 5;
    let (x, y, z) = (1,2,3);

    // function parameters
    let point = (3, 4);
    print_coordinates(&point);
}

// Function Parameters
fn foo(x: i32) {
    // code goes here
}
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y}");
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}
