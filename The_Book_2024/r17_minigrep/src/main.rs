// r17_minigrep
// Learning rust 2024, The Book §11, Command line tool
//
// 2024-12-01   PV

#![allow(dead_code, unused_variables)]

use std::{env, fs};

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

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}",config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");

}

struct Config {
    query: String,
    file_path: String,
}

// Passing a &Vec<String> to a &[String] is Ok...
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
