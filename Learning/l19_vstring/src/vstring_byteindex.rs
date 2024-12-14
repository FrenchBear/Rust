// vstrings library - byteindex based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions
//

#![allow(unused_mut)]

use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From byteindex

// ------------------------
// get byte

pub fn get_byte_from_byteindex(s: &str, byte_index: usize) -> u8 {
    s.as_bytes()[byte_index]
}

pub fn get_byteoption_from_byteindex(s: &str, byte_index: usize) -> Option<u8> {
    // Don't know which one is faster
    // s.as_bytes().get(index);
    s.bytes().nth(byte_index)
}

// ------------------------
// get char

pub fn get_char_from_byteindex(s: &str, byte_index: usize) -> char {
    s[byte_index..].chars().next().unwrap()
}

pub fn get_charoption_from_byteindex(s: &str, byte_index: usize) -> Option<char> {
    if byte_index >= s.len() {
        None
    } else {
        for (ix, ch) in s.char_indices() {
            if byte_index == ix {
                return Some(ch);
            }
            if byte_index > ix {
                return None;
            }
        }
        None
    }
}

// ------------------------
// get glyph

pub fn get_glyph_from_byteindex(s: &str, byte_index: usize) -> Glyph2 {
    get_glyphresult_from_byteindex(s, byte_index, true).unwrap()
}

pub fn get_glyphoption_from_byteindex(s: &str, byte_index: usize) -> Option<Glyph2> {
    get_glyphresult_from_byteindex(s, byte_index, false)
}

// Private base function
fn get_glyphresult_from_byteindex(s: &str, byte_index: usize, should_panic: bool) -> Option<Glyph2> {
    if byte_index >= s.len() {
        if should_panic {
            panic!("index out of bounds: the len is {} but the index is {}", s.len(), byte_index);
        } else {
            return None;
        }
    }

    for g in Glyph2::glyph2_indices(s) {
        if byte_index == *g.byte_range.start() {
            return Some(g);
        }
        if byte_index <= *g.byte_range.end() {
            if should_panic {
                // Similar panic message when we try to slice a str in the middle of multibyte UTF-8 character
                panic!(
                    "byte index {} is not a glyph boundary; it is inside '{}' (bytes {}..={})",
                    byte_index,
                    &s[g.byte_range.clone()],
                    *g.byte_range.start(),
                    *g.byte_range.end()
                );
            }
            return None;
        }
    }
    None // Actually we should never get here
}

// ------------------------
// get byte slice

pub fn get_byteslice_from_byteindex(s: &str, byte_index: usize) -> &[u8] {
    &s.as_bytes()[byte_index..=byte_index]
}

// ------------------------
// get byte vector

pub fn get_bytevector_from_byteindex(s: &str, byte_index: usize) -> Vec<u8> {
    vec![s.as_bytes()[byte_index]]
}

// ------------------------
// get char vector

pub fn get_charvector_from_byteindex(s: &str, byte_index: usize) -> Vec<char> {
    vec![s[byte_index..].chars().next().unwrap()]
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_byteindex(s: &str, byte_index: usize) -> Vec<Glyph2> {
    vec![get_glyph_from_byteindex(s, byte_index)]
}
