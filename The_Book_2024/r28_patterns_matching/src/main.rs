// r28_patterns_matching
// Learning rust 2024, Patterns & matching
//
// 2025-02-15   PV

#![allow(unused)]

fn main() {

    // match arms, mut be exhaustive
    let x = Some(5);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // Conditional if let, does not need to be exhaustive
    // Can mix if let, else, else if let and else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // While let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // For loops
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // let statements (let PATTERN = EXPRESSION
    let (x, y, z) = (1, 2, 3);

    // Function parameters (also work with closures)
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }
    let point = (3, 5);
    print_coordinates(&point);
    
}
