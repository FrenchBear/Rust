// r16_adder
// Learning rust 2024, The Book §11, Automated tests
//
// 2024-11-24   PV

#![allow(dead_code, unused_variables)]

pub fn add(left: u64, right: u64) -> u64 {
    // cargo test captures output to stdout, only print output if test fails, or use cargo test -- --show-output
    println!("Add {} and {}", left, right);

    left + right
}

// Tested from external tests folder
pub fn add_two(n: u64) -> u64 {
    n+2
}

// Because these tests are in the same files as the code, [cfg(test)]  specify that they shouldn’t be included in the compiled result
// Not needed if tests are located in a serarate tests folder (at the same level as src)
#[cfg(test)]
mod adder_tests {
    use super::*; // Need to bring in outer scope

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[should_panic(expected="this test fail")]     // Specifying a substring of expected panic message is optional
    #[test]
    fn a_function_that_panics() {
        panic!("Make this test fail");
    }

    // A test function can also return a Result<T, E>, Ok() is pass, Err(String) is fail
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}

// -----------------------------------------------------------

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*; // Need to bring in outer scope

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}

// -----------------------------------------------------------

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod greeting_tests {
    use super::*; // Need to bring in outer scope

    #[test]
    #[ignore]   // Ignored (typically for expensive tests), unless we use cargo test -- --ignored
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // With informative error message
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{result}`");
    }
}
