// l09_ponic
// Learning Rust again
//

// 2023-06-18   PV

#![allow(dead_code, unused_variables)]

use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

const SOURCE: &str = "C:\\Development\\GitHub\\Rust\\Learning\\l09_panic\\src\\main.rs";

fn main() {
    //do_panic();
    //panic_if_file_not_found();
    //unwrap_with_closures();

    let txt_result = read_text_2();
    if txt_result.is_ok() {
        let txt = txt_result.unwrap();
        println!("{txt}")
    }

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

// Can change main return to Result<(), E>
// Box<dyn Error> to mean “any kind of error”
fn main_2() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn do_panic() {
    // .expect() panics with a specified error message in case of error
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn panic_if_file_not_found() {
    // .unwrap() returns value Ok, and will panic in case of error (but better use .expect() instead to specify panic error message)
    let mut file =
        File::open("C:\\Development\\GitHub\\Rust\\Learning\\l09_panic\\src\\main.rs").unwrap();

    let mut s = String::new();
    let res = file.read_to_string(&mut s);
    if res.is_err() {
        panic!("Error reading file");
    }
    println!("{s}");
}

fn read_text() -> Result<String, io::Error> {
    let file_result = File::open(SOURCE);
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut txt = String::new();

    match file.read_to_string(&mut txt) {
        Ok(_) => Ok(txt),
        Err(e) => Err(e),
    } // No ; it's return value...
}

// Shorter version using ? operator
fn read_text_2() -> Result<String, io::Error> {
    let mut file = File::open(SOURCE)?;
    let mut txt = String::new();
    file.read_to_string(&mut txt)?;
    Ok(txt)
}

// Chaining ? operator
fn read_text_3() -> Result<String, io::Error> {
    let mut txt = String::new();
    File::open(SOURCE)?.read_to_string(&mut txt)?;
    Ok(txt)
}

// Even shorted, read_to_string(filename) does the whole thing!
fn read_text_4() -> Result<String, io::Error> {
    fs::read_to_string(SOURCE)
}

fn unwrap_with_closures() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            let f = File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            });
            print!("Fichier créé");
            f
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// Using ? with option
// This function returns Option<char> because it’s possible that there is a character there, but it’s also possible that there isn’t.
// This code takes the text string slice argument and calls the lines method on it, which returns an iterator over the lines in the string.
// Because this function wants to examine the first line, it calls next on the iterator to get the first value from the iterator.
// If text is the empty string, this call to next will return None, in which case we use ? to stop and return None from last_char_of_first_line.
// If text is not the empty string, next will return a Some value containing a string slice of the first line in text.
// The ? extracts the string slice, and we can call chars on that string slice to get an iterator of its characters.
// We’re interested in the last character in this first line, so we call last to return the last item in the iterator.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function
// that returns Option, but you can’t mix and match. The ? operator won’t automatically convert a Result to an Option or vice versa;
// in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.
