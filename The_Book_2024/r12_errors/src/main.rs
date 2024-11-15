// r12_errors
// Learning rust 2024, The Book §9, errors
//
// 2024-11-15   PV

#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::{self, Read};

fn main() {
    let res=read_username_from_file();
    match res {
        Ok(name) => println!("Name: {name}"),
        Err(err) => println!("Err: {:?}", err),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open(r"C:\Temp\hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // Last expression, retur,ed value
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
