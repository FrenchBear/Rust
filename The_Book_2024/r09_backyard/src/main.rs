// r09_backyard, main.rs
// Learning rust 2024, The Book §7 example
//
// 2024-11-10   PV

#![allow(dead_code, unused_variables)]

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
