use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = parse_config(&args);

    println!("Provided query: {}", config.query);
    println!("Filename to search in: {}", config.filename);

    let mut f = File::open(config.filename).expect("File not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed during reading file!");

    println!("File content:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {
        query,
        filename
    }
}