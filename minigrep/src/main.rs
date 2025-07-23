use std::env;
use std::fs;

fn main() {
    // get argument ex: cargo run -- needle haystack
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filepath);

    let contents = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
