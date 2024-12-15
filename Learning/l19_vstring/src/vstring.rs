// vstrings lib module
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions

#![allow(unused_mut, unused_imports)]

#[path = "vstring_byteindex.rs"]
pub mod vstring_byteindex;
pub use vstring_byteindex::*;

#[path = "vstring_byterange.rs"]
pub mod vstring_byterange;
pub use vstring_byterange::*;

#[path = "vstring_charindex.rs"]
pub mod vstring_charindex;
pub use vstring_charindex::*;

use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// Counting

pub fn get_byte_length(s: &str) -> usize {
    s.len()
}

pub fn get_char_length(s: &str) -> usize {
    s.chars().count()
}

pub fn get_glyph_length(s: &str) -> usize {
    Glyph2::glyph2_indices(s).count()
}

// ==========================================================================================
// Misc helpers

// Returns str from bytes slice
pub fn get_strref_from_byteslice<'a>(bytes: &'a [u8]) -> &'a str {
    str::from_utf8(bytes).unwrap()
}
