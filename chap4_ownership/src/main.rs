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
} // here z goes out of scope then str. but because str's value was moved, nothing special happen

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // here some_string goes out scope and `drop` is called. the backing memory is freed

fn makes_copy(some_integer: i32){ // some_integer comes into scope
    println!("{some_integer}");
} // here some_integer goes out scope. Nothing special happen
