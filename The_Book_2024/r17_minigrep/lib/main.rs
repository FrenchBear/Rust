// r17_minigrep - lib.rs
// Learning rust 2024, The Book §11, Command line tool
//
// 2024-12-07   PV

#![allow(dead_code, unused_variables)]

use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Passing a &Vec<String> to a &[String] is Ok...
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!  Usage: minigrep pattern file");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    //println!("With text:\n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
