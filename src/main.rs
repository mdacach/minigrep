use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query_string = &args[1];
    let filename = &args[2];
    println!("querying {} in file {}", query_string, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("Contents: \n{}", contents);
}
