//  library - char range based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-12-16   PV
// 2025-04-21   PV      Clippy optimizations

#![allow(unused_mut)]

use core::ops::Range;
use std::ops::RangeBounds;
use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From char range

// ------------------------
// Helpers

use std::ops::Bound::*;

#[derive(Debug, PartialEq)]
pub struct ByteCharRange {
    pub byte_range: Range<usize>,
    pub char_range: Range<usize>,
}

// Checks that char_range is compatible with s, accepts all variations of Range
// Panics in case of invalid range or incompatibility
// If Ok, return both a byte range and a char range Range<usize> representing all forms of ranges
pub fn validate_charrange<R>(s: &str, char_range: R) -> ByteCharRange
where
    R: RangeBounds<usize>,
{
    let count = s.chars().count();
    let startcharindex = match char_range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n + 1,
        Unbounded => 0,
    };
    let endcharindex = match char_range.end_bound() {
        Included(&n) => n + 1,
        Excluded(&n) => n,
        Unbounded => count,
    };

    if startcharindex > endcharindex {
        panic!("Invalid char range, start {} is greater than end {}", startcharindex, endcharindex);
    }
    if startcharindex > count {
        panic!("Invalid char range, start {} is greater than char count {}", startcharindex, count);
    }
    if endcharindex > count {
        panic!("Invalid char range, end {} is greater than char count {}", endcharindex, count);
    }

    // Convert char indexes into bytes indexes
    let mut startbyteindex = s.len();
    let mut endbyteindex = s.len();
    for (charindex, (ix, _)) in s.char_indices().enumerate() {
        if charindex == startcharindex {
            startbyteindex = ix;
        }
        if charindex == endcharindex {
            endbyteindex = ix;
            break;
        }
    }

    ByteCharRange {
        byte_range: startbyteindex..endbyteindex,
        char_range: startcharindex..endcharindex,
    }
}

// ------------------------
// get byte slice

// Simple implementation, panics if range is invalid or goes beyond s limits
pub fn get_byteslice_from_charrange<R>(s: &str, char_range: R) -> &[u8]
where
    R: RangeBounds<usize>,
{
    s[validate_charrange(s, char_range).byte_range].as_bytes()
}

// Specialized variants to extract by slice at the beginning or at the end
pub fn get_byteslice_from_startcharcount(s: &str, char_count: usize) -> &[u8] {
    get_byteslice_from_charrange(s, 0..char_count)
}

pub fn get_byteslice_from_endcharcount(s: &str, char_count: usize) -> &[u8] {
    get_byteslice_from_charrange(s, s.chars().count() - char_count..)
}

// ------------------------
// get byte vector, copying bytes

// Returning a Vec<u8> is Ok, but it duplicates characters
pub fn get_bytevector_from_charrange<R>(s: &str, char_range: R) -> Vec<u8>
where
    R: RangeBounds<usize>,
{
    (s[validate_charrange(s, char_range).byte_range]).bytes().collect()
}

// ------------------------
// get char vector

pub fn get_charvector_from_charrange<R>(s: &str, char_range: R) -> Vec<char>
where
    R: RangeBounds<usize>,
{
    //Vec::from_iter(s[validate_charrange(s, char_range)].chars())
    s[validate_charrange(s, char_range).byte_range].chars().collect()
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_charrange<R>(s: &str, char_range: R) -> Vec<Glyph2>
where
    R: RangeBounds<usize>,
{
    // Validate range and convert all variant into inclusive byte indexes for start and end
    // Don't need matching bytes range, argument char_range is enough to analyze Glyph2 values returned by iterator
    let r = validate_charrange(s, char_range);

    let mut accumulate = false;
    let mut res = Vec::new();
    for g in Glyph2::glyph2_indices(s) {
        if r.char_range.start == g.char_range.start {
            accumulate = true;
        };

        if r.char_range.start > g.char_range.start && r.char_range.start < g.char_range.end {
            // Similar panic message when we try to slice a str in the middle of multibyte UTF-8 character
            panic!(
                "Char range start {} is not a glyph boundary; it is inside glyph '{}' (characters {}..{}, bytes {}..{})",
                r.char_range.start,
                &s[g.byte_range.clone()],
                g.char_range.start,
                g.char_range.end,
                g.byte_range.start,
                g.byte_range.end
            );
        }
        if r.char_range.end > g.char_range.start && r.char_range.end < g.char_range.end {
            panic!(
                "Char range end {} is not a glyph boundary; it is inside glyph '{}' (characters {}..{}, bytes {}..{})",
                r.char_range.end,
                &s[g.byte_range.clone()],
                g.char_range.start,
                g.char_range.end,
                g.byte_range.start,
                g.byte_range.end
            );
        }

        if accumulate {
            let e = g.char_range.end;
            res.push(g);
            if r.char_range.end == e {
                return res;
            }
        }
    }
    panic!("Internal error, see https://xkcd.com/2200/"); // Should bever get here actually
}

// ------------------------
// get byte iterator

pub fn get_byteiterator_from_charrange<R>(s: &str, char_range: R) -> impl Iterator<Item = u8>
where
    R: RangeBounds<usize>,
{
    s[validate_charrange(s, char_range).byte_range].bytes()
}

// ------------------------
// get char iterator

pub fn get_chariterator_from_charrange<R>(s: &str, char_range: R) -> impl Iterator<Item = char>
where
    R: RangeBounds<usize>,
{
    s[validate_charrange(s, char_range).byte_range].chars()
}

// ------------------------
// get glyph iterator

pub fn get_glyphiterator_from_charrange<R>(s: &str, char_range: R) -> impl Iterator<Item = Glyph2>
where
    R: RangeBounds<usize>,
{
    get_glyphvector_from_charrange(s, char_range).into_iter()
}

// ------------------------
// get &str

pub fn get_strref_from_charrange<R>(s: &str, char_range: R) -> &str
where
    R: RangeBounds<usize>,
{
    &s[validate_charrange(s, char_range).byte_range]
}

// ------------------------
// get String

pub fn get_string_from_charrange<R>(s: &str, char_range: R) -> String
where
    R: RangeBounds<usize>,
{
    s[validate_charrange(s, char_range).byte_range].to_string()
}
