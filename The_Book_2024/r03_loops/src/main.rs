// r03_loops
// Play with loops in rust
// In rust, break can return a value for a block, and/or specify a loop label to indicate wich look should break
//
// 2024-11-04   PV
// 2025-04-21   PV      Clippy suggestions

#![allow(clippy::never_loop)]

fn main() {
    let mut count = 0;
    'counting_up: loop {
        // Labelled loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Exit current loop
            }
            if count == 2 {
                break 'counting_up; // Exit two levels of loops
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}\n");

    // Combine returning a value with break and specifying a label
    let res = 'main_loop: loop {
        let mut i = 0;
        loop {
            {
                i += 1;
                if i == 4 {
                    break 'main_loop 12;
                }
            }
        }
    };
    println!("res={res}\n");

    // While loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        if index == 3 {
            break;
        }
        index += 1;
    }
    println!();

    // For loop
    for element in (1..5).rev() {
        println!("the value is: {element}");
    }
}
