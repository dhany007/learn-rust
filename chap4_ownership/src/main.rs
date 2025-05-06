fn main() {
    println!("Understanding ownership");
    // Ownership is a set of rules that govern how a Rust program manages memory

    let x = String::from("hello");
    let y = x;
    // println!("x = {x}"); // error

    println!("clone = copy the heap data of the String");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    println!("do not use copy");
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}");
    // some of the types that implement Copy:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    let str = String::from("hello"); // str become this scope
    take_ownership(str); // str's value move into function
                        // ... and so is no valid here

    // println!("str = {str}"); // error

    let z = 5; // z come into this scope
    makes_copy(z); // because i32 implement the Copy trait,
                    // z does NOT move into this function
    println!("z = {z}"); // so it's okay z to move afterward

    println!("Return values and scope");
    let str1 = gives_ownership(); // gives_ownership moves its return value into str1
    println!("str1 = {str1}");

    let str2 = String::from("hello"); // str2 comes into this scope
    println!("str2 = {str2}");

    let str3 = takes_and_gives_back(str2); // str2 moved into  takes_and_gives_back,
                                            // which is moves its return value into str3
    println!("str3 = {str3}");

    println!("return multiple values using a tuple");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // here some_string goes out scope and `drop` is called. the backing memory is freed

fn makes_copy(some_integer: i32){ // some_integer comes into scope
    println!("{some_integer}");
} // here some_integer goes out scope. Nothing special happen

fn gives_ownership() -> String { // give_ownership will move it's return value into the function who call it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();

    (some_string, length)
}