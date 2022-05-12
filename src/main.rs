use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = minigrep::run(config);
    if let Err(e) = result {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
