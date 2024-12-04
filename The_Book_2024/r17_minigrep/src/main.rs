// r17_minigrep
// Learning rust 2024, The Book §11, Command line tool
//
// 2024-12-01   PV

#![allow(dead_code, unused_variables)]

use std::{env, fs, process};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    // Other collections that can be produced by collect():
    /*
    let a1: VecDeque<String> = env::args().collect();
    dbg!(a1);

    let a2: LinkedList<String> = env::args().collect();
    dbg!(a2);

    let a3: HashSet<String> = env::args().collect();
    dbg!(a3);

    let a4: BTreeSet<String> = env::args().collect();
    dbg!(a4);

    let a5: HashMap<String, bool> = env::args().map(|a| (a, true)).collect();
    dbg!(a5);

    let a6: BTreeMap<String, bool> = env::args().map(|a| (a, true)).collect();
    dbg!(a6);
    */

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("With text:\n{contents}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Passing a &Vec<String> to a &[String] is Ok...
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!  Usage: minigrep pattern file");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }
}
