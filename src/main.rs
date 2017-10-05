use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (query, filename) = parse_args(&args);

    println!("Provided query: {}", query);
    println!("Filename to search in: {}", filename);

    let mut f = File::open(filename).expect("File not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed during reading file!");

    println!("File content:\n{}", contents);
}


fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}