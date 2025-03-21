// l27_cfg: Learning Rust, conditional compilation, features, build script
//
// 2025-03-21	PV      First version

#![allow(dead_code, unused_variables)]

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
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

fn main() {
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    println!("pi: {}", pi());

    #[cfg(feature = "myfeature")]
    println!("nyfeature is avtive")
}
