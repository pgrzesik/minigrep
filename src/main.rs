use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Provided query: {}", query);
    println!("Filename to search in: {}", filename);

    let mut f = File::open(filename).expect("File not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed during reading file!");

    println!("File content: {}", contents);
}
