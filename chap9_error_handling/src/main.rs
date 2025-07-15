use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Error Handling");

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

/*
    T represents the type of the value that will be returned in a success case within the Ok variant,
    and E represents the type of the error
    that will be returned in a failure case within the Err variant
 */
enum Result<T, E> {
    Ok(T),
    Err(E),
}
