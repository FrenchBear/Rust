// l14_grep
// Learning Rust again
//
// 2023-06-25   PV

#![allow(unused)]

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );

    let contents = fs::read_to_string(config.file_path).expect("Reading file failed");

    println!("Text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Usage: grep pattern file");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: String::from(&args[2]),
        })
    }
}
