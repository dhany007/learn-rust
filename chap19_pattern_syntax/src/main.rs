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
}

struct Point {
    x: i32,
    y: i32,
}
