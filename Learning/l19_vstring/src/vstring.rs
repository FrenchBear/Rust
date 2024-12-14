// vstrings lib module
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions
//
// Two strings in rust:
// - str:    in the core language, usually seen in its borrowed form &str. String slices are references to some UTF-8 encoded string data stored elsewhere.
//           String literals, for example, are stored in the program’s binary and are therefore string slices.
// - String: Provided by Rust’s standard library is a growable, mutable, owned, UTF-8 encoded string type.
//           Many of the same operations available with Vec<T> are available with String as well because String is actually implemented as a wrapper
//           around a vector of bytes Vec<u8> with some extra guarantees, restrictions, and capabilities.

#![allow(unused_mut)]

use core::ops::{Range, RangeInclusive};
use std::ops::{RangeFrom, RangeFull, RangeTo, RangeToInclusive};
use std::str;

use crate::glyph::Glyph;

// ==========================================================================================
// Counting

pub fn get_byte_length(s: &str) -> usize {
    s.len()
}

pub fn get_char_length(s: &str) -> usize {
    s.chars().count()
}

pub fn get_glyph_length(s: &str) -> usize {
    Glyph::glyph_indices(s).count()
}

// ==========================================================================================
// Misc helpers

// Returns str from bytes slice
pub fn get_strref_from_byteslice<'a>(bytes: &'a [u8]) -> &'a str {
    str::from_utf8(bytes).unwrap()
}

// ==========================================================================================
// From byteindex

pub fn get_byte_from_byteindex(s: &str, byte_index: usize) -> u8 {
    s.as_bytes()[byte_index]
}

pub fn get_byteoption_from_byteindex(s: &str, byte_index: usize) -> Option<u8> {
    // Don't know which one is faster
    // s.as_bytes().get(index);
    s.bytes().nth(byte_index)
}

pub fn get_char_from_byteindex(s: &str, byte_index: usize) -> char {
    s[byte_index..].chars().next().unwrap()
}

pub fn get_charoption_from_byteindex(s: &str, byte_index: usize) -> Option<char> {
    if byte_index >= s.len() {
        None
    } else {
        s[byte_index..].chars().next()
    }
}

pub fn get_glyph_from_byteindex(s: &str, byte_index: usize) -> Glyph {
    Glyph::glyph_indices(&s[byte_index..]).next().unwrap().1
}

pub fn get_glyphoption_from_byteindex(s: &str, byte_index: usize) -> Option<Glyph> {
    if byte_index >= s.len() {
        None
    } else {
        // There's probably a more direct to convert a Option<(X,Y)> into an (Option<X>, (Option<Y>) (functional laanguages have it...)
        // Actually, asked Gemini, he proposed basically the same code in rust
        let xx = Glyph::glyph_indices(&s[byte_index..]).next();
        if let Some((_, g)) = xx {
            Some(g)
        } else {
            None
        }
    }
}

// ----------------------------------
// Returns slice of bytes

// Can only return a reference to an array, can't return directly an array since its size is now known at compile time
// It's efficient since the return is just a slice pointing on provided &str (hence the need for lifetime tagging)
// need range.clone() since slicing consumes the original range
pub fn get_byteslice_from_range<'a>(s: &'a str, range: &Range<usize>) -> &'a [u8] {
    &s.as_bytes()[range.clone()]
}

// Variant with a range inclusive: start..=end
pub fn get_byteslice_from_rangeinclusive<'a>(s: &'a str, range: &RangeInclusive<usize>) -> &'a [u8] {
    &s.as_bytes()[range.clone()]
}

// Variant, range from: start..
pub fn get_byteslice_from_rangefrom<'a>(s: &'a str, range: &RangeFrom<usize>) -> &'a [u8] {
    &s.as_bytes()[range.clone()]
}

// Variant, range to: ..end
pub fn get_byteslice_from_rangeto<'a>(s: &'a str, range: &RangeTo<usize>) -> &'a [u8] {
    &s.as_bytes()[range.clone()]
}

// Variant, range to inclusive: ..=end
pub fn get_byteslice_from_rangetoinclusive<'a>(s: &'a str, range: &RangeToInclusive<usize>) -> &'a [u8] {
    &s.as_bytes()[range.clone()]
}

// Variant with a full range = ..
pub fn get_byteslice_from_rangerangefull<'a>(s: &'a str, _: &RangeFull) -> &'a [u8] {
    &s.as_bytes()[..]
}

// Variant, no range
pub fn get_byteslice<'a>(s: &'a str) -> &'a [u8] {
    &s.as_bytes()[..]
}

pub fn get_byteslice_from_start<'a>(s: &'a str, length: usize) -> &'a [u8] {
    &s.as_bytes()[0..length]
}

pub fn get_byteslice_from_end<'a>(s: &'a str, length: usize) -> &'a [u8] {
    &s.as_bytes()[s.len() - length..]
}

// ----------------------------------
// Returns vector Vec<u8>, copying bytes

pub fn get_bytevector(s: &str) -> Vec<u8> {
    Vec::from(s.as_bytes())
}

// Returning a Vec<u8> is Ok, but it duplicates characters
pub fn get_bytevector_from_range(s: &str, range: &Range<usize>) -> Vec<u8> {
    Vec::from(&s.as_bytes()[range.clone()])
}

pub fn get_bytevector_from_rangeinclusive(s: &str, range: &RangeInclusive<usize>) -> Vec<u8> {
    Vec::from(&s.as_bytes()[range.clone()])
}

// can add many variants

// ----------------------------------
// Returns iterator over bytes (easier to use than iterator over &u8)

// Returning an iterator on bytes
pub fn get_byteiterator_from_range<'a>(s: &'a str, range: &Range<usize>) -> impl Iterator<Item = u8> + 'a {
    s.as_bytes()[range.clone()].iter().copied()
}

// // Probably not efficient to handle &u8...
// pub fn get_byterefiterator_from_range<'a>(s: &'a str, range: &Range<usize>) -> impl Iterator<Item = &'a u8> {
//     s.as_bytes()[range.clone()].iter()
// }

pub fn get_byteiterator_from_rangeinclusive<'a>(s: &'a str, range: &RangeInclusive<usize>) -> impl Iterator<Item = u8> + 'a {
    s.as_bytes()[range.clone()].iter().copied()
}

// and many variants

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
    let start = *range.start();
    let end = *range.end() + 1;
    let newrange = start..end;
    get_refstr_from_charindexrange(s, &newrange)
}
*/

// ==========================================================================================
// Glyph functions
