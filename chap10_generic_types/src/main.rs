fn main() {
    println!("Generic Types");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.1, y: 2.2 };

    // don't work because mix integer and float, should be same struct T
    let wont_work = Point { x: 1, y: 2.2 };

    // work because have generic T and U
    let integer_and_float = Point { x: 1, y: 2.2 };
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic in struct definition
// type of T is must be same
struct Point<T> {
    x: T,
    y: T,
}

// can different because have T and U
struct Point2<T, U> {
    x: T,
    y: U,
}

// generic in enum definition
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}