// R06_enum
// Learning Rust
// 2018-10-22	PV

// To kill warnings about unused variants at global level
#![allow(dead_code)]

#[derive(Debug)]
enum PrimaryColors {
    Red,
    Blue,
    Green,
}

// Import enums in current scope using a glob to avoid use of PrimaryColors:: prefix
use PrimaryColors::*;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

impl Message {
    fn hello(&self) {
        println!("Hello");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Iowa,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let c1 = PrimaryColors::Red;
    println!("c1: {:?}", c1);

    let c2 = (Red, Green, Blue);
    println!("c2: {:?}", c2);

    let m = Message::Write(String::from("Bonjour"));
    m.hello();

    let mut some_number = Some(5);
    let mut no_number: Option<i32> = None;

    // match with option<T>
    some_number = increment(some_number);
    no_number = increment(no_number);

    let sum = 3 + some_number.unwrap() + no_number.unwrap_or(0);
    println!("sum: {}", sum);

    let q = Coin::Quarter(UsState::Iowa);
    let v = value_in_cents(&q);
    println!("v: {}", v);

    // Use of _ as a default case.  Use () to return unity value ≃ empty
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // same thing using if let (use = and not =>)
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// match with option<T>
fn increment(n: Option<i32>) -> Option<i32> {
    match n {
        Some(rn) => Some(rn + 1),
        None => None,
    }
}

// match with an enum
fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Quarter(state) => {                   // binding pattern: fill variable state
            println!("Quater of {:?}", state);
            25
        }
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
    }
}
