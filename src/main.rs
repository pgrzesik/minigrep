use std::env;
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("During parsing arguments, experienced error: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("During application execution, following error occured: {}", e);
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("File content:\n{}", contents);

    Ok(())
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
