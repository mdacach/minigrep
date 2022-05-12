use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query_string = &args[1];
    let filename = &args[2];
    println!("querying {} in file {}", query_string, filename);
}
