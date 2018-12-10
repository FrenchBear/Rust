// R13_minigrep, a rust version of grep
// Learning rust
// 2018-11-30	PV


use std::env;
use std::fs;
use std::process;
//use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", &config.query);
    println!("In file {}", &config.filename);

    let contents = fs::read_to_string(&config.filename).expect(&format!("Problem reading file {}", &config.filename));
    println!("File contents:\n{}", contents);

}
