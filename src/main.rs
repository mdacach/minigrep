use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = run(config);
    if let Err(e) = result {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// Dependency Injection
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Contents: \n{}", contents);

    Ok(())
}

struct Config {
    query_string: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query_string = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query_string,
            filename,
        })
    }
}
