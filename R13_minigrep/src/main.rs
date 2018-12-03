// R13_minigrep, a rust version of grep
// Learning rust
// 2018-11-30	PV


use std::env;
use std::fs;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()!=3 {
        panic!("Usage: minigrep string file");
    }

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect(&format!("Problem reading file {}", filename)[..]);
    println!("File contents:\n{}", contents);

}
