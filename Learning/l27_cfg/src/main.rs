// l27_cfg: Learning Rust, conditional compilation, features, build script
//
// 2025-03-21	PV      First version

#![allow(dead_code, unused_variables)]

// This function only gets compiled if the target OS is Linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* Linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[cfg(pi4)]
fn pi() -> f64 {
    4.0
}

#[cfg(not(pi4))]
fn pi() -> f64 {
    3.1415926536
}

// Different code paths can be conditionally compiled based on the panic setting. The current values available are unwind and abort.
// The panic strategy can be set from the command line by using abort or unwind.
// rustc lemonade.rs -C panic=abort

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    if cfg!(target_os = "windows") {
        println!("More specifically, it's Windows");
    }

    println!("pi: {}", pi());

    #[cfg(feature = "myfeature")]
    println!("nyfeature is avtive");

    println!();

    drink("water");
    drink("lemonade");
}
