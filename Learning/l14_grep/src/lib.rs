// l14_grep
// Learning Rust again
//
// 2023-06-25   PV

#![allow(unused)]

use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut nargs = 0;
        let mut query = &String::from("");
        let mut file_path= &String::from("");
        let mut ignore_case = false;
        for arg in &args[1..] {
            if (arg.to_lowercase() == "-i" || arg.to_lowercase() == "/i") {
                ignore_case = true;
            } else {
                match nargs {
                    0 => {
                        query = arg;
                        nargs += 1;
                    }
                    1 => {
                        file_path = arg;
                        nargs += 1;
                    }
                    _ => return Err("Too many orguments"),
                }
            }
        }
        if nargs != 2 {
            return Err("Usage: l14_grep [-i] pattern file");
        }

        let c = Config {
            query: query.to_string(),
            file_path: file_path.clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok() | ignore_case,
        };
        //dbg!(&c);

        Ok(c)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in (if config.ignore_case {
        search_case_insensitive
    } else {
        search
    })(&config.query, &contents)
    {
        println!("{line}")
    }
    Ok(())
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
    let query = query.to_lowercase(); // Type changes from &str to String!
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // The backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        // The backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
