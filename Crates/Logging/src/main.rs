// Loggin crate test app
// Quick-and-dirty main function to test code during dev
//
// 2025-05-05   PV      First version
// 2025-09-15   PV      Debugging info test

//#![allow(unused)]

use logging::*;
use std::{fs, io};

use dirs as _;
use chrono as _;
use colored as _;

fn main() -> io::Result<()> {
    println!("Crate version: {}\n", logging::version());

    let mut lw = logging::new("test", "1.1.0", true);
    logln(&mut lw, "Hello");
    logln(&mut lw, "*** Error: message");
    logln(&mut lw, "dbg: Debugging info");

    let file_path = lw.get_path().unwrap();
    let content = fs::read_to_string(file_path.as_path())?;

    println!("\n\nLog File path: {}", file_path.display());
    println!("Log File content:\n{}", content);

    Ok(())
}
