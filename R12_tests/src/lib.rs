// Learning Rust
// Tests
// 2018-11-27	PV

#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length>other.length && self.width>other.width
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value<1 || value>100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length:10, width:8};
        let smaller = Rectangle { length:5, width: 4};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let r1 = Rectangle{length: 3, width:5};
        let r2 = Rectangle{length: 6, width:6 };
        assert!(!r1.can_hold(&r2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Pierre");
        assert!(result.contains("Piere"), "Greeting did not contain name, value was `{}`", result);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
