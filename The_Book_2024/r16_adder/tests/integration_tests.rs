// r16_adder
// Learning rust 2024, The Book §11, Automated tests
// Example of tests located in tests folder
//
// 2024-11-24   PV

use r16_adder::add_two;

pub fn setup() {
    // setup code specific to your library's tests would go here
}

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}

