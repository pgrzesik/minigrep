extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("During parsing arguments, experienced error: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("During application execution, following error occured: {}", e);
        process::exit(1)
    }
}
