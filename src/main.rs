use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("During parsing arguments, experienced error: {}", err);
        process::exit(1)
    });

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("File not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Failed during reading file!");

    println!("File content:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided!")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
