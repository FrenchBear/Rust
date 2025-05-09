// l14_grep
// Learning Rust again
//
// 2023-06-25   PV
// 2023-06-28   PV      Use iterators from chap 15

//#![allow(unused)]

use std::env;
use std::process;

use l14_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        //let config = Config::build_using_iterator(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = l14_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
