// r12_errors
// Learning rust 2024, The Book §9, errors
//
// 2024-11-15   PV

#![allow(dead_code, unused_variables)]

use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    let tf: [(i32, fn() -> Result<String, io::Error>); 4] = [
        (1, read_username_from_file1),
        (2, read_username_from_file2),
        (3, read_username_from_file3),
        (4, read_username_from_file4),
    ];

    for (n, f) in &tf {
        let res = f();
        match res {
            Ok(name) => println!("{n}: Name: {name}"),
            Err(err) => println!("{n}: Err: {:?}", err),
        }
    }

    test_option(fn_opt1);
    test_option(fn_opt2);

    let g = test_module::Guess::new(45);
    //g.value = 50;     // private (because it's in a separate module, otherwise it would be accessible)
    println!("g: {}", g.value());
}

// Note that main() can also return a Result<(), E>
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }

// use std::process::ExitCode;
// fn main() -> ExitCode {
//     // Your program logic here...
//     ExitCode::from(12)
// }

fn read_username_from_file1() -> Result<String, io::Error> {
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

// Use ? shortcut to unwrap Ok result or rethrow Err result
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open(r"C:\Temp\hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Can chain after ? with Ok result
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(r"C:\Temp\hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Even more compact version with fs::read_to_string that take a filename as parameter, open it, creae a new String and
// read file content into it, and return it as Ok result, or an io::Error if something fails
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string(r"C:\Temp\hello.txt")
}

fn test_option(f: fn(&[i32]) -> Option<i32>) {
    let a: [i32; 5] = [0, 1, 2, 3, 4];
    let res = f(&a);
    match res {
        Some(i) => println!("res -> {i}"),
        None => println!("res -> None"),
    }
}

fn fn_opt1(t: &[i32]) -> Option<i32> {
    let n1 = t.get(1)?;
    let n2 = t.get(7)?;
    Some(n1 + n2)
}

fn fn_opt2(t: &[i32]) -> Option<i32> {
    Some(t.get(1)? + t.get(7)?)
}

// Alt exammple, both next() and chars() are iterators, and both next() and last() return as Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
    // let c1 = Option::<char>::None;
    // let c2 = Option::<&str>::None;
    // let c3: Option<char> = Option::<&str>::None;     // Mismatched types error

    // But this works, despite the fact that next() returns an Optin<&'a str>
    text.lines().next()?.chars().last()
}

// ----------------------------------------
// A Guess type that will only continue with values between 1 and 100

mod test_module {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }

            Guess { value }
        }

        // getter, because value field is private
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
