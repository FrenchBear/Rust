// Loggin crate test app
// Quick-and-dirty main function to test code during dev
//
// 2025-05-05   PV      First version

//#![allow(unused)]

use logging::*;

use dirs as _;
use chrono as _;
use colored as _;

fn main() {
    println!("Crate version: {}\n", logging::version());

    let mut lw = logging::new("test", "1.0.0", true);
    logln(&mut lw, "Hello");
}
