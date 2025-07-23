use std::env;

fn main() {
    // get argument ex: cargo run -- needle haystack
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filepath);
}
