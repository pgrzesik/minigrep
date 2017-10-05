use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Provided query: {}", query);
    println!("Filename to search in: {}", filename);
}
