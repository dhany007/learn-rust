use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // get argument ex: cargo run -- needle haystack
    let args: Vec<String> = env::args().collect();

    // that unwrap_or_else will pass the inner value of the Err, which in this case is the static string "not enough arguments"
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
