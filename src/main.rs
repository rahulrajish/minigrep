use std::env;
use std::process;

use minigrep01::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments : {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("in File {}", config.file_path);

    if let Err(e) = minigrep01::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}