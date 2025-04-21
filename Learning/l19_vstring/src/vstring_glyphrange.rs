// vstrings library - gluphrange based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-12-18   PV
// 2025-04-21   PV      Clippy optimizations

#![allow(unused_mut)]

use core::ops::Range;
use std::ops::RangeBounds;
use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From charrange

use std::ops::Bound::*;

#[derive(Debug, PartialEq)]
pub struct ByteCharGlyphRange {
    pub byte_range: Range<usize>,
    pub char_range: Range<usize>,
    pub glyph_range: Range<usize>,
}

// Checks that glyph_range is compatible with s, accepts all variations of Range
// Panics in case of invalid range or incompatibility
// If Ok, return both a byte range, a char range and a gluph range Range<usize> representing all forms of ranges
pub fn validate_glyphrange<R>(s: &str, glyph_range: R) -> ByteCharGlyphRange
where
    R: RangeBounds<usize>,
{
    let glyphcount = Glyph2::glyph2_indices(s).count();
    let startglyphindex = match glyph_range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n + 1,
        Unbounded => 0,
    };
    let endglyphindex = match glyph_range.end_bound() {
        Included(&n) => n + 1,
        Excluded(&n) => n,
        Unbounded => glyphcount,
    };

    if startglyphindex > endglyphindex {
        panic!(
            "Invalid glyph range, start {} is greater than end {}",
            startglyphindex, endglyphindex
        );
    }
    if startglyphindex > glyphcount {
        panic!(
            "Invalid glyph range, start {} is greater than glyph count {}",
            startglyphindex, glyphcount
        );
    }
    if endglyphindex > glyphcount {
        panic!(
            "Invalid glyph range, end {} is greater than glyph count {}",
            endglyphindex, glyphcount
        );
    }

    // Convert glyph indexes into bytes and char indexes
    let mut startbyteindex = s.len();
    let mut endbyteindex = s.len();
    let mut startcharindex = s.chars().count();
    let mut endcharindex = startcharindex;

    for (glyphindex, g) in Glyph2::glyph2_indices(s).enumerate() {
        if glyphindex == startglyphindex {
            startbyteindex = g.byte_range.start;
            startcharindex = g.char_range.start;
        }
        if glyphindex == endglyphindex {
            endbyteindex = g.byte_range.start; // Since we're at the character after the last one, we use start values to fill end/byte indexes
            endcharindex = g.char_range.start;
            break;
        }
    }

    ByteCharGlyphRange {
        byte_range: startbyteindex..endbyteindex,
        char_range: startcharindex..endcharindex,
        glyph_range: startglyphindex..endglyphindex,
    }
}

// ------------------------
// get byte slice

// Simple implementation, panicks if range is invalid or goes beyond s limits
pub fn get_byteslice_from_glyphrange<R>(s: &str, glyph_range: R) -> &[u8]
where
    R: RangeBounds<usize>,
{
    s[validate_glyphrange(s, glyph_range).byte_range].as_bytes()
}

// Specialized variants to extract by slice at the beginning or at the end
pub fn get_byteslice_from_startglyphcount(s: &str, glyph_count: usize) -> &[u8] {
    get_byteslice_from_glyphrange(s, 0..glyph_count)
}

pub fn get_byteslice_from_endglyphcount(s: &str, glyph_count: usize) -> &[u8] {
    get_byteslice_from_glyphrange(s, Glyph2::glyph2_indices(s).count() - glyph_count..)
}

// ------------------------
// get byte vector, copying bytes

// Returning a Vec<u8> is Ok, but it duplicates characters
pub fn get_bytevector_from_glyphrange<R>(s: &str, glyph_range: R) -> Vec<u8>
where
    R: RangeBounds<usize>,
{
    // ToDo: Check which version is the most efficient
    //Vec::from_iter((&s[validate_glyphrange(s, glyph_range)]).bytes())
    (s[validate_glyphrange(s, glyph_range).byte_range])
        .bytes()
        .collect()
}

// ------------------------
// get char vector

pub fn get_charvector_from_glyphrange<R>(s: &str, glyph_range: R) -> Vec<char>
where
    R: RangeBounds<usize>,
{
    //Vec::from_iter(s[validate_glyphrange(s, glyph_range)].chars())
    s[validate_glyphrange(s, glyph_range).byte_range]
        .chars()
        .collect()
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_glyphrange<R>(s: &str, glyph_range: R) -> Vec<Glyph2>
where
    R: RangeBounds<usize>,
{
    // Validate range and convert all variants into Range<usize>
    let r = validate_glyphrange(s, glyph_range);

    let mut accumulate = false;
    let mut res = Vec::new();
    for (ix, g) in Glyph2::glyph2_indices(s).enumerate() {
        if r.glyph_range.start == ix {
            accumulate = true;
        };

        if accumulate {
            res.push(g);
            if r.glyph_range.end == ix + 1 {
                return res;
            }
        }
    }
    panic!("Internal error, see https://xkcd.com/2200/"); // Should bever get here actually
}

// ------------------------
// get byte iterator

pub fn get_byteiterator_from_glyphrange<R>(
    s: &str,
    glyph_range: R,
) -> impl Iterator<Item = u8>
where
    R: RangeBounds<usize>,
{
    s[validate_glyphrange(s, glyph_range).byte_range].bytes()
}

// ------------------------
// get char iterator

pub fn get_chariterator_from_glyphrange<R>(
    s: &str,
    glyph_range: R,
) -> impl Iterator<Item = char>
where
    R: RangeBounds<usize>,
{
    s[validate_glyphrange(s, glyph_range).byte_range].chars()
}

// ------------------------
// get glyph iterator

pub fn get_glyphiterator_from_glyphrange<R>(s: &str, glyph_range: R) -> impl Iterator<Item = Glyph2>
where
    R: RangeBounds<usize>,
{
    get_glyphvector_from_glyphrange(s, glyph_range).into_iter()
}

// ------------------------
// get &str

pub fn get_strref_from_glyphrange<R>(s: &str, glyph_range: R) -> &str
where
    R: RangeBounds<usize>,
{
    &s[validate_glyphrange(s, glyph_range).byte_range]
}

// ------------------------
// get String

pub fn get_string_from_glyphrange<R>(s: &str, glyph_range: R) -> String
where
    R: RangeBounds<usize>,
{
    s[validate_glyphrange(s, glyph_range).byte_range].to_string()
}
