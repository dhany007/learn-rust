use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // that unwrap_or_else will pass the inner value of the Err, which in this case is the static string "not enough arguments"
    // env::args() return an iterator
    let config = Config::build(env::args()).unwrap_or_else(|err| {
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
