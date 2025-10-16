fn main() {
    println!("Matching Literals!");
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("Matching Named Variables");
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    println!("Multiple Patterns");
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("Matching Ranges of Values with ..=");
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    println!("Destructuring to Break Apart Values");
    let p = Point {x: 5, y: 10};
    let Point{x: a, y: b} = p;
    assert_eq!(5, a);
    assert_eq!(10, b);

    let p = Point {x: 0, y: 7};
    let Point{x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point {x: 0, y: 7};

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})")
        },
    }

    println!("Destructuring Enums");
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}")
        }
        Message::Write(text) => {
            println!("Text message: {text}")
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }

    println!("Destructuring Nested Structs and Enums");
    let msg2 = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg2{
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
        NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, {s}, and {v}")
        }
        _ => ()
    }

    println!("Ignoring values in pattern");
    foo(3,4);

    let number = (1,2,3,4,5);
    match number {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    println!("an unused variable by starting its name with _");
    let _x = 10;
    let y = 5;

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");

    println!("remaining parts of value with ..");
    let origin = Point2{x: 0, y: 0, z: 0};
    match origin {
        Point2{x, ..} => { // The syntax .. will expand to as many values as it needs to be
            println!("x is {x}");
        }
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first,..,last) => {
            println!("Some numbers: {first} {last}")
        }
    }

    println!("extra conditionals in match guard");
    // match guard is an additional if condition, specified after the pattern match
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => {
            println!("the number {x} is even")
        }
        Some(x) => {
            println!("the number {x} is odd")
        }
        _ => ()
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => {
            println!("Got 50")
        }
        Some(n) if n==y => {
            println!("Matched, n = {n}")
        }
        _ => {
            println!("default case, x = {x:?}")
        }
    }
    println!("at the end; x = {x:?}, y = {y}");

    let x = 4;
    let y = false;


    match x {
        // The match condition states that the arm only matches
        // if the value of x is equal to 4, 5, or 6 and if y is true
        2 | 3 | 4 if y => {
            println!("yes")
        }
        _ => println!("no")
    }

    println!("@ Binding");
    // The at operator @ lets us create a variable that holds a value at the same time we’re testing
    // that value for a pattern match

    let message = Message3::Hello {id: 5};
    match message {
        Message3::Hello {
            id: id_variable @ 3..=7, // seolah2 dari 3 sampai 7 kita buat di variable id_variable
        } => {
            println!("Found an id in range: {id_variable}") // Found an id in range: 5
        }
        Message3::Hello {
            id: 10..=12
        } => {
            println!("Found an id in another range")
        }
        Message3::Hello {
            id
        } => {
            println!("Found some other id: {id}")
        }
    }
}

fn foo (_:i32, y:i32) {
    println!("The value of y is {y}");
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NewMessage {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message3 {
    Hello { id: i32}
}