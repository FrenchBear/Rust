// l55_panic_hook
// Intercept a panic in current thread in rust
// While you can't truly "stop" the panic in the same scope, you can potentially prevent the entire program from crashing
// if the panic happens within a spawned thread.
//
// 2025_04_28   PV

#![allow(unused)]

use std::panic;

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("Custom panic handler called!");
        if let Some(location) = panic_info.location() {
            eprintln!("Panic occurred in file '{}' at line {}", location.file(), location.line());
        } else {
            eprintln!("Panic occurred but location information is not available.");
        }
        if let Some(payload) = panic_info.payload().downcast_ref::<&'static str>() {
            eprintln!("Panic message: {}", payload);
        } else if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("Panic message: {}", payload);
        } else {
            eprintln!("Panic payload is not a string.");
        }
        // You could potentially perform cleanup or other actions here.
    }));

    println!("Program starting...");
    panic!("Something went terribly wrong!");
    println!("This line will not be reached.");
}

