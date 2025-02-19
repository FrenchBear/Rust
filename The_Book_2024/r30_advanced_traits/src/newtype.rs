// Advanced traits, Newtype pattern: implement aernal trait on an external type
//
// 2025-02-18   PV

use std::{fmt, ops::Deref};

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// With deref, can easily access to all methods of Vec<String> on *Wrapper
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn main() {
    println!("\nNewtype pattern");
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    let res = format!("[{}]", w.0.iter().map(|x| format!("{:#?}", x)).collect::<Vec<String>>().join(", "));
    println!("w = {res}");
}
