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

fn prints_and_returns_10(n: i32) -> i32 {
    println!("Received value {}", n);
    10
}


pub fn add_two(n:i32) -> i32 {
    n+2
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
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
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Pierre");
        assert!(result.contains("Piere"), "Greeting did not contain name, value was `{}`", result);
    }

    // Example of #[should_panic]
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // This test is based on Result<T,E>, can't use #[should_panic], but return an error instead
    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2==4 {
            Ok(())
        } else {
            Err(String::from("2+2!=4"))
        }
    }

    // Tests with some output.  Note that function is not public, that's fine with Rust.
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // Executed if -- --ignored is passed
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

}
