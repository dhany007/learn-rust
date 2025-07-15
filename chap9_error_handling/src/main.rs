use std::fs::File;

fn main() {
    println!("Error Handling");

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
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
