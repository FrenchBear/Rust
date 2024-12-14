// vstrings lib module
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions

#![allow(unused_mut)]

#[path = "vstring_byteindex.rs"]
mod vstring_byteindex;
pub use vstring_byteindex::*;

#[path = "vstring_byterange.rs"]
mod vstring_byterange;
pub use vstring_byterange::*;


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

// ==========================================================================================
// Char functions

pub fn get_char_from_charindex(s: &str, char_index: usize) -> char {
    s.chars().nth(char_index).unwrap()
}

pub fn get_charoption_from_charindex(s: &str, char_: usize) -> Option<char> {
    s.chars().nth(char_)
}

// Returns String from bytes slice (c'ant return &str since bytes do not exist in &[char])
pub fn get_string_from_charslice(chars: &[char]) -> String {
    chars.iter().collect::<String>()
}

// ----------------------------------
// Returns slice of chars

/*
pub fn get_refstr_from_charindexrange<'a>(s: &'a str, range: &Range<usize>) -> &'a str {
    let start = s.char_indices().nth(range.start).unwrap().0;
    let end = if range.end == get_char_length(s) {
        get_byte_length(s)
    } else {
        s.char_indices().nth(range.end).unwrap().0
    };
    &s[start..end]
}

pub fn get_charslice_from_charindexrange<'a>(s: &'a str, range: &Range<usize>) -> &'a[char] {
    let start = s.char_indices().nth(range.start).unwrap().0;
    let end = if range.end == get_char_length(s) {
        get_byte_length(s)
    } else {
        s.char_indices().nth(range.end).unwrap().0
    };
    let zz = (&s[start..end]);
    let s = zz.chars().collect::<Vec<char>>();

}

pub fn get_refstr_from_charindexrangeinclusive<'a>(s: &'a str, range: &RangeInclusive<usize>) -> &'a str {
    let start = *range.start();     // No need to call function
    let end = *range.end() + 1;
    let newrange = start..end;
    get_refstr_from_charindexrange(s, &newrange)
}
*/

// ==========================================================================================
// Glyph functions
