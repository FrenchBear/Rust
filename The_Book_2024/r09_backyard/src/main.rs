// r09_backyard, main.rs
// Learning rust 2024, The Book §7 example
//
// 2024-11-10   PV

#![allow(dead_code, unused_variables)]

use crate::garden::vegetables::Asparagus;

pub mod field;
pub mod garden; // Required, since module field is defined in field/mod.rs, there's no mod statement in mod.rs, so can't make it pub from here
// Without that, field module is private and not accessible

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    let corn_field = field::Field {
        crop: String::from("corn"),
        surface: 20.0,
    };
    println!("{:?}", corn_field);
}
