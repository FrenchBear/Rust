// vstring library - glyph index based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-12-17   PV
// 2025-04-21   PV      Clippy optimizations

#![allow(unused_mut)]

use core::ops::Range;
use core::panic;
use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From charindex

// Helper, returns a Glyph2 for a given glyph_index, or panics if index is not valid
pub fn validate_glyphindex(s: &str, glyph_index: usize) -> Glyph2 {
    let mut ix = 0;
    let mut it = Glyph2::glyph2_indices(s);

    loop {
        let glopt = it.next();
        if glopt.is_none() {
            panic!(
                "glyph index out of bounds: &str contains {} glyph(s), but the index is {}",
                ix, glyph_index
            );
        }
        if ix == glyph_index {
            return glopt.unwrap();
        }
        ix += 1;
    }
}

// ------------------------
// get glyph

pub fn get_glyph_from_glyphindex(s: &str, glyph_index: usize) -> Glyph2 {
    validate_glyphindex(s, glyph_index)
}

pub fn get_glyphoption_from_glyphindex(s: &str, glyph_index: usize) -> Option<Glyph2> {
    for (ix, g) in Glyph2::glyph2_indices(s).enumerate() {
        if ix == glyph_index {
            return Some(g);
        }
    }
    None
}

// ------------------------
// get byte slice
pub fn get_byteslice_from_glyphindex(s: &str, glyph_index: usize) -> &[u8] {
    &s.as_bytes()[validate_glyphindex(s, glyph_index).byte_range]
}

// ------------------------
// get byte vector

pub fn get_bytevector_from_glyphindex(s: &str, glyph_index: usize) -> Vec<u8> {
    Vec::from(&s[validate_glyphindex(s, glyph_index).byte_range])
}

// ------------------------
// get char vector

pub fn get_charvector_from_glyphindex(s: &str, glyph_index: usize) -> Vec<char> {
    Vec::from_iter((s[validate_glyphindex(s, glyph_index).byte_range]).chars())
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_glyphindex(s: &str, glyph_index: usize) -> Vec<Glyph2> {
    vec![get_glyph_from_glyphindex(s, glyph_index)]
}

// ------------------------
// get byte iterator

pub fn get_byteiterator_from_glyphindex(
    s: &str,
    glyph_index: usize,
) -> impl Iterator<Item = u8> {
    s[validate_glyphindex(s, glyph_index).byte_range].bytes()
}

// ------------------------
// get char iterator

pub fn get_chariterator_from_glyphindex(
    s: &str,
    glyph_index: usize,
) -> impl Iterator<Item = char> {
    Vec::from_iter((s[validate_glyphindex(s, glyph_index).byte_range]).chars()).into_iter()
}

// ------------------------
// get glyph iterator

pub fn get_glyphiterator_from_glyphindex(
    s: &str,
    glyph_index: usize,
) -> impl Iterator<Item = Glyph2> {
    get_glyphvector_from_glyphindex(s, glyph_index).into_iter()
}

// ------------------------
// get str ref

pub fn get_strref_from_glyphindex(s: &str, glyph_index: usize) -> &str {
    &s[validate_glyphindex(s, glyph_index).byte_range]
}

// ------------------------
// get string

pub fn get_string_from_glyphindex(s: &str, glyph_index: usize) -> String {
    s[validate_glyphindex(s, glyph_index).byte_range].to_string()
}
