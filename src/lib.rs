use std::error::Error;
use std::fs;

pub struct Config {
    pub query_string: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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

// Dependency Injection
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Contents: \n{}", contents);

    Ok(())
}
