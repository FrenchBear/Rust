// R09_errors
// Learning Rust
// 2018-11-10	PV

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    //test_panic();
    test_error_1();
    test_error_2();
    test_error_3();
    let r = read_username_from_file_1();
    let r = read_username_from_file_2();
    divzero();
}

fn test_panic() -> i32 {
    let v = vec![1, 2, 3];
    v[99]
}

// T represents the type of the value that will be returned in a success case within the Ok variant, and E represents
// the type of the error that will be returned in a failure case within the Err variant.
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;

const HELLO_FILE: &str = r"c:\temp\hello.txt";

fn test_error_1() {
    let f = match File::open(HELLO_FILE) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(HELLO_FILE) {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}

fn test_error_2() {
    let f = File::open(HELLO_FILE).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(HELLO_FILE).unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

// Shortcuts unwrap and expect
fn test_error_3() {
    let f = File::open(HELLO_FILE).unwrap(); // get file or panic with std msg "called `Result::unwrap()`"
    let f = File::open(HELLO_FILE).expect("can't open hello.txt"); // get file or panic with a specific message
}

// propagating errors
// 'Long' version
use std::io;
use std::io::Read;
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Same functionality as previous fn
// The ? operator can only be used in functions that have a return type of Result<T, E>
// If result is a Ok(value), unwraps value, otherwise function returns Error (after conversion if needed and available) 
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even more compact, chaining calls after ?, still the same behavior
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


fn divzero() {
    test_division(12, 4);
    test_division(4, 0);
}

// Use of checked_div to return a Option<u32> (and not a Result<u32, DivisioByZeroError>) to control division by zero
fn test_division(numerator: u32, denominator: u32) {
    match numerator.checked_div(denominator) {
        Some(result) => println!("{} / {} = {}", numerator, denominator, result),
        None => println!("{} / {} results in a division by zero", numerator, denominator)
    }
}
