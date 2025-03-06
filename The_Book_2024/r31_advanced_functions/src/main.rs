// r31_advanced_functions
// Learning rust 2024, Advanced features
//
// 2025-03-01   PV

#![allow(unused)]


fn add_one(x: i32) -> i32 {
    x + 1
}

// Paramet of type function pointer
// Do not confuse fn type with Fn trait!
// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce)
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}


// Function returning a closure
// Closures are represented by traits --> dyn Fn. They're not sized, sp wrat it in Box<>
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
