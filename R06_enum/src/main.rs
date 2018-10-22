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

fn main() {
    let c1 = PrimaryColors::Red;
    println!("c1: {:?}", c1);

    let m = Message::Write(String::from("Bonjour"));
    m.hello();

    let some_number = Some(5);
    let no_number: Option<i32> = None;

    let sum = 3 + some_number.unwrap() + no_number.unwrap_or(0);
    println!("sum: {}", sum);
}
