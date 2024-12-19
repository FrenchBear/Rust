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

#[path = "vstring_charrange.rs"]
pub mod vstring_charrange;
pub use vstring_charrange::*;

#[path = "vstring_glyphindex.rs"]
pub mod vstring_glyphindex;
pub use vstring_glyphindex::*;

#[path = "vstring_glyphrange.rs"]
pub mod vstring_glyphrange;
pub use vstring_glyphrange::*;


use core::str;

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
// Conversion to &str

// Returns &str from bytes slice
pub fn get_strref_from_byteslice<'a>(byteslice: &'a [u8]) -> &'a str {
    str::from_utf8(byteslice).unwrap()
}

// Returns str from bytes vector
pub fn get_strref_from_bytevector<'a>(bytevector: &'a Vec<u8>) -> &'a str {
    str::from_utf8(bytevector.as_slice()).unwrap()
}

// ==========================================================================================
// Conversion to String

// Returns String from byte slice &[u8]
pub fn get_string_from_byteslice(byteslice: &[u8]) -> String {
    //String::from_utf8(byteslice.to_vec()).unwrap()
    //str::from_utf8(byteslice).unwrap().to_string()
    String::from(str::from_utf8(byteslice).unwrap())
}

// Returns String from byte vector Vec<u8> (takes ownership of vector)
pub fn get_string_from_bytevector(bytevector: Vec<u8>) -> String {
    String::from_utf8(bytevector).unwrap()
}

// Returns String from byte vector ref &vec[u8]
pub fn get_string_from_bytevectorref(bytevectorref: &Vec<u8>) -> String {
    //String::from_utf8(bytevectorref.clone()).unwrap()       // Inefficient because of clone()
    //str::from_utf8(bytevectorref).unwrap().to_string()
    String::from(str::from_utf8(bytevectorref).unwrap())
}

// Returns String from byte iterator
pub fn get_string_from_byteiterator(byteiterator: impl Iterator<Item = u8>) -> String {
    String::from_utf8(byteiterator.collect::<Vec<u8>>()).unwrap()
}

// DOn't know if unsafe variant is faster
pub unsafe  fn get_string_from_byteiterator_unsafe(byteiterator: impl Iterator<Item = u8>) -> String {
    String::from_utf8_unchecked(byteiterator.collect::<Vec<u8>>())
}

// ----

// Returns String from slice &[char]
pub fn get_string_from_charslice(charslice: &[char]) -> String {
    //charslice.iter().collect()
    String::from_iter(charslice)
}

// Returns String from char vector, takes ownership
pub fn get_string_from_charvector(charvector: Vec<char>) -> String {
    //charvector.iter().collect()
    String::from_iter(charvector)
}

// Returns String from char vector ref
pub fn get_string_from_charvectorref(charvectorref: &Vec<char>) -> String {
    //charvectorref.iter().collect()
    String::from_iter(charvectorref)
}

// Returns String from char iterator
pub fn get_string_from_chariterator(chariterator: impl Iterator<Item = char>) -> String {
    //chariterator.collect()
    String::from_iter(chariterator)
}
