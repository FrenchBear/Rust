// r11_kollections
// Learning rust 2024, The Book §8, common collections
//
// 2024-11-10   PV

#![allow(dead_code, unused_variables)]

pub mod vectors;
pub mod strings;
pub mod hashmaps;

fn main() {
    vectors::test_vectors();
    strings::test_strings();
    hashmaps::test_hashmaps();
}
