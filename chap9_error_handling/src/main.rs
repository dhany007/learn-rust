use std::fs::File;
use std::io::{ErrorKind, self, Read};
use std::fs;

fn main() {
    println!("Error Handling");

}

/*
    T represents the type of the value that will be returned in a success case within the Ok variant,
    and E represents the type of the error
    that will be returned in a failure case within the Err variant
 */
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// using unwrap_or_else
fn alternative() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn recoverable() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") { // if error not found, we create file
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                panic!("Problem opening the file: {:?}", error)
            }
        },
    };
}

fn unwrap() {
    //  { code: 2, kind: NotFound, message: "No such file or directory" }
    let greeting_file2 = File::open("hello.txt").unwrap();
}

fn expect() {
    // hello.txt should be included in this project:
    // Os { code: 2, kind: NotFound, message: "No such file or directory" }
    let greeting_file3 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

/*
    propagating can return the error to the calling code so that it can decide what to do
 */

// Result<T, E>
// Result<String, io::Error>
// the generic parameter T has been filled in with the concrete type String
// and the generic type E has been filled in with the concrete type io::Error.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

// chaining method calls immediately after the ?
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}


// using fs
/*
    fs::read_to_string function that opens the file,
    creates a new String,
    reads the contents of the file,
    puts the contents into that String,
    and returns it
 */
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/*
    we’re only allowed to use the ? operator in a function
    that returns Result, Option, or another type that implements FromResidual.
 */
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


/*
    To panic! or Not to panic!
    - When code panics, there’s no way to recover.
    - When you choose to return a Result value, you give the calling code options
    - Returning Result is a good default choice when you’re defining a function that might fail.
    -
 */



/*
The panic! macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values
The Result enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from
 */
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}