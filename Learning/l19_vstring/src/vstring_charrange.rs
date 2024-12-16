// vstrings library - charrange based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-12-16   PV

#![allow(unused_mut)]

use core::ops::Range;
use std::ops::RangeBounds;
use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From charrange

// ------------------------
// Helpers

use std::ops::Bound::*;

// Checks that char_range is compatible with s, accepts all variations of Range
// Panics in case of invalid range or incompatibility
// If Ok, return a bytes index Range<usize> representing all forms of ranges
pub fn validate_charrange<R>(s: &str, char_range: R) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let count = s.chars().count();
    let start = match char_range.start_bound() {
        Included(&n) => n,
        Excluded(&n) => n + 1,
        Unbounded => 0,
    };
    let end = match char_range.end_bound() {
        Included(&n) => n + 1,
        Excluded(&n) => n,
        Unbounded => count,
    };

    if start > end {
        panic!("Invalid range, start {} is greater than end {}", start, end);
    }
    if start > count {
        panic!("Invalid range, start {} is greater than chars count {}", start, count);
    }
    if end > count {
        panic!("Invalid range, end {} is greater than chars count {}", end, count);
    }

    // Convert char indexes into bytes indexes
    let mut startbyteindex = s.len();
    let mut endbyteindex = s.len();
    let mut charindex = 0;
    for (ix, _) in s.char_indices() {
        if charindex == start {
            startbyteindex = ix;
        }
        if charindex == end {
            endbyteindex = ix;
        }
        charindex += 1;
    }

    startbyteindex..endbyteindex
}

// ------------------------
// get byte slice

// Simple implementation, panicks if range is invalid or goes beyond s limits
pub fn get_byteslice_from_charrange<'a, R>(s: &'a str, char_range: R) -> &'a [u8]
where
    R: RangeBounds<usize>,
{
    &s[validate_charrange(s, char_range)].as_bytes()
}

// Specialized variants to extract by slice at the beginning or at the end
pub fn get_byteslice_from_startcharcount<'a>(s: &'a str, char_count: usize) -> &'a [u8] {
    get_byteslice_from_charrange(s, 0..char_count)
}

pub fn get_byteslice_from_endcharcount<'a>(s: &'a str, char_count: usize) -> &'a [u8] {
    get_byteslice_from_charrange(s, s.chars().count() - char_count..)
}

// ------------------------
// get byte vector, copying bytes

/*
// Returning a Vec<u8> is Ok, but it duplicates characters
pub fn get_bytevector_from_charrange<R>(s: &str, char_range: R) -> Vec<u8>
where
    R: RangeBounds<usize>,
{
    // ToDo: Check which version is the most efficient
    //Vec::from(&s.as_bytes()[validate_charrange(s, char_range)])
    s[validate_charrange(s, char_range)].bytes().collect()
}

// ------------------------
// get char vector

pub fn get_charvector_from_charrange<R>(s: &str, char_range: R) -> Vec<char>
where
    R: RangeBounds<usize>,
{
    //Vec::from_iter(s[validate_charrange(s, char_range)].chars());
    s[validate_charrange(s, char_range)].chars().collect()
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_charrange<R>(s: &str, char_range: R) -> Vec<Glyph2>
where R: RangeBounds<usize>,
{
    // Validate range and convert all varians into inclusive byte indexes for start and end
    let r = validate_charrange(s, char_range);

    let mut accumulate = false;
    let mut res = Vec::new();
    for g in Glyph2::glyph2_indices(s) {
        if r.start == g.char_range.start {
            accumulate = true;
        };

        if r.start > g.char_range.start && r.start < g.char_range.end {
            // Similar panic message when we try to slice a str in the middle of multibyte UTF-8 character
            panic!(
                "Range.start {} is not a glyph boundary; it is inside '{}' (bytes {}..{})",
                r.start,
                &s[g.char_range.clone()],
                g.char_range.start,
                g.char_range.end
            );
        }

        if accumulate {
            let e = g.char_range.end;
            res.push(g);
            if r.end == e {
                return res;
            }
        }
    }
    panic!("Internal error, see https://xkcd.com/2200/"); // Should bever get here actually
}

// ------------------------
// get byte iterator

pub fn get_byteiterator_from_charrange<'a, R>(s: &'a str, char_range: R) -> impl Iterator<Item = u8> + 'a where R: RangeBounds<usize>, {
    s[validate_charrange(s, char_range)].bytes()
}

// ------------------------
// get char iterator

pub fn get_chariterator_from_charrange<'a, R>(s: &'a str, char_range: R) -> impl Iterator<Item = char> + 'a where R: RangeBounds<usize>, {
    s[validate_charrange(s, char_range)].chars()
}

// ------------------------
// get glyph iterator

pub fn get_glyphiterator_from_charrange<R>(s: &str, char_range: R) -> impl Iterator<Item = Glyph2> where R: RangeBounds<usize>, {
    get_glyphvector_from_charrange(s, char_range).into_iter()
}

// ------------------------
// get &str

pub fn get_strref_from_charrange<'a, R>(s: &'a str, char_range: R) -> &'a str
where
    R: RangeBounds<usize>,
{
    &s[validate_charrange(s, char_range)]
}

// ------------------------
// get String

pub fn get_string_from_charrange<R>(s: &str, char_range: R) -> String
where
    R: RangeBounds<usize>,
{
    s[validate_charrange(s, char_range)].to_string()
}

*/
