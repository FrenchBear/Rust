// vstrings library - charindex based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions
// 2025-04-21   PV      Clippy optimizations

#![allow(unused_mut)]

use core::ops::Range;
use core::panic;
use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From charindex

// Helper, returns byte index Range<usize> for a given char_index, or panics if index is not valid
pub fn validate_charindex(s: &str, char_index: usize) -> Range<usize> {
    let mut ix = 0;
    let mut it = s.char_indices();

    loop {
        let ciopt = it.next();
        if ciopt.is_none() {
            panic!(
                "char index out of bounds: &str contains {} character(s), but the index is {}",
                ix, char_index
            );
        }

        if ix == char_index {
            let ci = ciopt.unwrap();
            let nextopt = it.next();
            if let Some(item) = nextopt {
                return ci.0..item.0;
            } else {
                return ci.0..s.len();
            }
        }

        ix += 1;
    }
}

// ------------------------
// get char

pub fn get_char_from_charindex(s: &str, char_index: usize) -> char {
    (s[validate_charindex(s, char_index)])
        .chars()
        .next()
        .unwrap()
}

pub fn get_charoption_from_charindex(s: &str, char_index: usize) -> Option<char> {
    for (ix, char) in s.char_indices() {
        if ix == char_index {
            return Some(char);
        }
    }

    None
}

// ------------------------
// get glyph

pub fn get_glyph_from_charindex(s: &str, char_index: usize) -> Glyph2 {
    let mut charcount = 0;
    for g in Glyph2::glyph2_indices(s) {
        if g.char_range.start == char_index {
            return g;
        }
        charcount = g.char_range.end;
    }
    panic!(
        "char index out of bounds: s contains {} characters, but the index is {}",
        charcount, char_index
    );
}

pub fn get_glyphoption_from_charindex(s: &str, char_index: usize) -> Option<Glyph2> {
    // for g in Glyph2::glyph2_indices(s) {
    //     if g.char_range.start == char_index {
    //         return Some(g);
    //     }
    // }
    // None
    Glyph2::glyph2_indices(s).find(|g| g.char_range.start == char_index)
}

// ------------------------
// get byte slice

pub fn get_byteslice_from_charindex(s: &str, char_index: usize) -> &[u8] {
    &s.as_bytes()[validate_charindex(s, char_index)]
}

// ------------------------
// get byte vector

pub fn get_bytevector_from_charindex(s: &str, char_index: usize) -> Vec<u8> {
    Vec::from(&s[validate_charindex(s, char_index)])
}

// ------------------------
// get char vector

pub fn get_charvector_from_charindex(s: &str, char_index: usize) -> Vec<char> {
    Vec::from_iter((s[validate_charindex(s, char_index)]).chars())
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_charindex(s: &str, char_index: usize) -> Vec<Glyph2> {
    vec![get_glyph_from_charindex(s, char_index)]
}

// ------------------------
// get byte iterator

pub fn get_byteiterator_from_charindex(
    s: &str,
    char_index: usize,
) -> impl Iterator<Item = u8> {
    s[validate_charindex(s, char_index)].bytes()
}

// ------------------------
// get char iterator

pub fn get_chariterator_from_charindex(
    s: &str,
    char_index: usize,
) -> impl Iterator<Item = char>{
    Vec::from_iter((s[validate_charindex(s, char_index)]).chars()).into_iter()
}

// ------------------------
// get glyph iterator

pub fn get_glyphiterator_from_charindex(
    s: &str,
    char_index: usize,
) -> impl Iterator<Item = Glyph2> {
    get_glyphvector_from_charindex(s, char_index).into_iter()
}

// ------------------------
// get strref

pub fn get_strref_from_charindex(s: &str, char_index: usize) -> &str {
    &s[validate_charindex(s, char_index)]
}

// ------------------------
// get string

pub fn get_string_from_charindex(s: &str, char_index: usize) -> String {
    s[validate_charindex(s, char_index)].to_string()
}
