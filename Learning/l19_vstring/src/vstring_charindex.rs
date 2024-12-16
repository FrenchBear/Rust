// vstrings library - charindex based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions
//

#![allow(unused_mut)]

use core::ops::Range;
use core::panic;
use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From charindex

// Helper, returns Range<usize> for a given char_index, and panics if index is not valid

#[derive(Debug, PartialEq)]
pub struct ByteRangeChar{
    pub byte_range: Range<usize>,
    pub char: char,
}

pub fn validate_charindex(s: &str, char_index: usize) -> ByteRangeChar {
    let mut ix = 0;
    let mut it = s.char_indices();

    loop {
        let ciopt = it.next();
        if ciopt.is_none() {
            panic!("char index out of bounds: s contains {} characters, but the index is {}", ix, char_index);
        }

        if ix == char_index {
            let ci = ciopt.unwrap();
            let nextopt = it.next();
            if nextopt.is_none() {
                return ByteRangeChar { byte_range: ci.0..s.len(), char: ci.1};
            } else {
                return ByteRangeChar { byte_range:ci.0..nextopt.unwrap().0, char: ci.1};
            }
        }

        ix += 1;
    }
}

// ------------------------
// get char

pub fn get_char_from_charindex(s: &str, char_index: usize) -> char {
    validate_charindex(s, char_index).char
}

pub fn get_charoption_from_charindex(s: &str, char_index: usize) -> Option<char> {
    for (ix, char) in s.char_indices() {
        if ix == char_index {
            return Some(char);
        }
    }
    return None;
}

// ------------------------
// get glyph

pub fn get_glyph_from_charindex(s: &str, char_index: usize) -> Glyph2 {
    let mut charcount = 0;
    for g in Glyph2::glyph2_indices(s) {
        if g.char_range.start == char_index { return g; }
        charcount = g.char_range.end;
    }
    panic!("char index out of bounds: s contains {} characters, but the index is {}", charcount, char_index);
}

pub fn get_glyphoption_from_charindex(s: &str, char_index: usize) -> Option<Glyph2> {
    for g in Glyph2::glyph2_indices(s) {
        if g.char_range.start == char_index { return Some(g); }
    }
    None
}

// ------------------------
// get byte slice

pub fn get_byteslice_from_charindex(s: &str, char_index: usize) -> &[u8] {
    &s.as_bytes()[validate_charindex(s, char_index).byte_range]
}

// ------------------------
// get byte vector

pub fn get_bytevector_from_charindex(s: &str, char_index: usize) -> Vec<u8> {
    Vec::from(&s[validate_charindex(s, char_index).byte_range])
}

// ------------------------
// get char vector

pub fn get_charvector_from_charindex(s: &str, char_index: usize) -> Vec<char> {
    Vec::from_iter((&s[validate_charindex(s, char_index).byte_range]).chars())
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_charindex(s: &str, char_index: usize) -> Vec<Glyph2> {
    vec![get_glyph_from_charindex(s, char_index)]
}

// ------------------------
// get byte iterator

pub fn get_byteiterator_from_charindex<'a>(s: &'a str, char_index: usize) -> impl Iterator<Item = u8> + 'a {
    s[validate_charindex(s, char_index).byte_range].bytes()
}

// ------------------------
// get char iterator

pub fn get_chariterator_from_charindex<'a>(s: &'a str, char_index: usize) -> impl Iterator<Item = char> + 'a {
    Vec::from_iter((&s[validate_charindex(s, char_index).byte_range]).chars()).into_iter()
}

// ------------------------
// get glyph iterator

pub fn get_glyphiterator_from_charindex<'a>(s: &'a str, char_index: usize) -> impl Iterator<Item = Glyph2> + 'a {
    get_glyphvector_from_charindex(s, char_index).into_iter()
}

// ------------------------
// get strref

pub fn get_strref_from_charindex<'a>(s: &'a str, char_index: usize) -> &'a str {
    &s[validate_charindex(s, char_index).byte_range]
}

// ------------------------
// get string

pub fn get_string_from_charindex(s: &str, char_index: usize) -> String {
    s[validate_charindex(s, char_index).byte_range].to_string()
}
