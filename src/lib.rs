use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip first arg (executable)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Error while getting query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Error while getting filename string")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Some query";
        let content = "some content\nSome query in line\nand one more";

        assert_eq!(vec!["Some query in line"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "some query";
        let content = "some content\nSome query in line\nand one more";

        assert_eq!(vec!["Some query in line"], search_case_insensitive(query, content));
    }
}