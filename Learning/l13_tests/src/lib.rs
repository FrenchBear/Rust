// l12_tests
// Learning Rust again
//
// Use cargo test --help to show the options you can use with cargo test.
// When running tests (cargo test), output is not displayed by default, or use «cargo test -- --show-output»
// By default, tests run in parallel, you must make sure your tests don’t depend on each other or on any shared state, including a shared environment,
// such as the current working directory or environment variables. Or use «cargo test -- --test-threads=1» to disable parallelism.
// Use «cargo test -- --ignored» to run tests with #[ignore] attribute (ex: expensive tests)
//
// 2023-06-23   PV

#![allow(unused)]

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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    //format!("Hello {}!", name)
    format!("Hello")
}

// The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
// In this case, the configuration option is test, which is provided by Rust for compiling and running tests. By using the cfg attribute,
// Cargo compiles our test code only if we actively run the tests with cargo test. This includes any helper functions that might be within this module,
// in addition to the functions annotated with #[test].
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

// -----------------------

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    #[should_panic(expected="Guess value must be between 1 and 100, got 200.")] // Can optionally use expected="msg" to be sure that the function parics for the correct reason
    fn greater_than_100() {
        let g = Guess::new(200);
    }
}

// -----------------------

#[cfg(test)]
mod tests3 {
    use super::*;

    // A test function that has the Result<T, E> will pass if it returns T, and fails if it returns E.
    // Using operator ? is a convenient way to to fail the test if any operation within returns an Err variant.
    #[test]
    fn it_works() -> Result<(), String> {
        if add(2,2)==4 {
            Ok(())
        } else {
            Err(String::from("Adding 2 and 2 doesn't make 4!"))
        }
    }
}