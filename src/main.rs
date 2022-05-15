use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = minigrep::run(config);
    if let Err(e) = result {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
