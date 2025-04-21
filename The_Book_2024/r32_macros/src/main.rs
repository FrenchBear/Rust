// r32_macros
// Learning rust 2024, Advanced features, Procedural macros
//
// 2025-03-01   PV
// 2025-04-21   PV      Clippy suggestions

#![allow(clippy::vec_init_then_push)]

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*

            temp_vec
        }
    };
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

pub fn main() {
    let v = vec2![1, 2, 3];
    println!("v = {:?}", v);

    Pancakes::hello_macro();
}
