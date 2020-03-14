extern crate simple_error;

use simple_error::SimpleError;
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, SimpleError> {
        if args.len() < 3 {
            return Err(SimpleError::new("not enough args"));
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_insensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;
    let search_results = if config.case_insensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };

    for line in search_results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let query = "hi";
        let content = "hi\nGood boy.\nPlease let me know if I can do for you.\nThanks.";
        assert_eq!(vec!["hi"], search(query, content));
    }
    #[test]
    fn case_insensitive_search_test() {
        let query = "Hi";
        let content = "hi\nGood boy.\nPlease let me know if I can do for you.\nThanks.";
        assert_eq!(vec!["hi"], search_case_insensitive(query, content));
    }
}