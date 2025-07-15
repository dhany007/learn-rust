use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Error Handling");

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") { // if error not found, we create file
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {:?}", error)
    //         }
    //     },
    // };

    // unwrap
    //  { code: 2, kind: NotFound, message: "No such file or directory" }
    let greeting_file2 = File::open("hello.txt").unwrap();

    // expect
    let greeting_file3 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
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