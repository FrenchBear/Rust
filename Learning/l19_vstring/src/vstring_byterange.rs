// vstrings library - byterange based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions
//

#![allow(unused_mut)]

use core::ops::Range;
use std::ops::RangeBounds;
use std::str;

use crate::glyph2::Glyph2;

// ==========================================================================================
// From byterange

// ------------------------
// Helpers

// Checks that byte_range is compatible with a &str on len bytes
// Panics in case of invalid range or incompatibility
// If Ok, return a usize tuple(start_index, end_index_included) representing all forms of ranges
pub fn validate_byterange<R>(len: usize, byte_range: R) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let stnum = match byte_range.start_bound() {
        std::ops::Bound::Included(&st) => {
            if st >= len {
                panic!("Range.start {} is too large for s.len {}", st, len);
            };
            match byte_range.end_bound() {
                std::ops::Bound::Included(&enin) => {
                    if enin < st {
                        panic!("Invalid range, start {} is greater than end included {}", st, enin);
                    }
                }
                // An empty range x..x is accepted
                std::ops::Bound::Excluded(&enex) => {
                    if enex < st {
                        panic!("Invalid range, start {} is greater or equal to end excluded {}", st, enex);
                    }
                }
                std::ops::Bound::Unbounded => {}
            };
            st
        }
        std::ops::Bound::Excluded(_) => 0, // Don't think this case exists for start bound
        std::ops::Bound::Unbounded => 0,
    };

    let eninnum = match byte_range.end_bound() {
        std::ops::Bound::Included(&enin) => {
            if enin >= len {
                panic!("Range.end included {} is too large for s.len {}", enin, len);
            };
            enin + 1
        }
        std::ops::Bound::Excluded(&enex) => {
            if enex > len {
                panic!("Range.end excluded {} is too large for s.len {}", enex, len);
            };
            enex
        }
        std::ops::Bound::Unbounded => len,
    };

    stnum..eninnum
}

// ------------------------
// get byte slice

// Simple implementation, panicks if range is invalid or goes beyond s limits
pub fn get_byteslice_from_byterange<'a, R>(s: &'a str, byte_range: R) -> &'a [u8]
where
    R: RangeBounds<usize>,
{
    &s[validate_byterange(s.len(), byte_range)].as_bytes()
}

// Explore variants to return results in case of errors: basic version that panics, Option<&[u8]>, Result<&[u8], String>,
// and a tolerant version that always return a &[u8]. This will not be done for all other cases
// Here is a simplified implementation, only accepting a Range (and not RangeIncluded for instance)

// Option<&[u8]> variant, returns None in cases causing basic version to panic, and Some<&[u8]> if Ok
pub fn get_bytesliceoption_from_byterange<'a>(s: &'a str, byte_range: Range<usize>) -> Option<&'a [u8]> {
    if byte_range.start > byte_range.end || byte_range.start > s.len() || byte_range.end > s.len() {
        return None;
    }
    Some(&s[byte_range].as_bytes())
}

// Result<&[u8],String> varant, return an error string in cases causing basic version to panic, ok Ok(&[u8])
pub fn get_bytesliceresult_from_byterange<'a>(s: &'a str, byte_range: Range<usize>) -> Result<&'a [u8], String> {
    if byte_range.start > byte_range.end {
        return Err(format!(
            "Invalid range, start {} is greater than end {}",
            byte_range.start, byte_range.end
        ));
    }
    if byte_range.start >= s.len() {
        return Err(format!("Range.start {} is too large for s.len {}", byte_range.start, s.len()));
    }
    if byte_range.end > s.len() {
        return Err(format!("Range.end {} is too large for s.len {}", byte_range.end, s.len()));
    }
    Ok(&s[byte_range].as_bytes())
}

// Tolerant version, in invalid cases or range beyond s limits, return empty byte slice or limit range to actual s bounds
pub fn get_byteslicetolerant_from_byterange<'a>(s: &'a str, byte_range: Range<usize>) -> &'a [u8] {
    // Invalid range or start after s end, return an empty byte slice
    if byte_range.start >= byte_range.end || byte_range.start >= s.len() {
        return b"";
    }

    // Ensure that the actual range end is clipped to s.len()
    let en = if byte_range.end > s.len() { s.len() } else { byte_range.end };
    &s[byte_range.start..en].as_bytes()
}

// Specialized variants to extract by slice at the beginning or at the end
pub fn get_byteslice_from_startbytecount<'a>(s: &'a str, byte_count: usize) -> &'a [u8] {
    get_byteslice_from_byterange(s, 0..byte_count)
}

pub fn get_byteslice_from_endbytecount<'a>(s: &'a str, byte_count: usize) -> &'a [u8] {
    get_byteslice_from_byterange(s, s.len() - byte_count..)
}

// ------------------------
// get byte vector, copying bytes

// Returning a Vec<u8> is Ok, but it duplicates characters
pub fn get_bytevector_from_byterange<R>(s: &str, byte_range: R) -> Vec<u8>
where
    R: RangeBounds<usize>,
{
    // ToDo: Check which version is the most efficient
    //Vec::from(&s.as_bytes()[validate_byterange(s.len(), byte_range)])
    s[validate_byterange(s.len(), byte_range)].bytes().collect()
}

// ------------------------
// get char vector

pub fn get_charvector_from_byterange<R>(s: &str, byte_range: R) -> Vec<char>
where
    R: RangeBounds<usize>,
{
    //Vec::from_iter(s[validate_byterange(s.len(), byte_range)].chars());
    s[validate_byterange(s.len(), byte_range)].chars().collect()
}

// ------------------------
// get glyph vector

pub fn get_glyphvector_from_byterange<R>(s: &str, byte_range: R) -> Vec<Glyph2>
where R: RangeBounds<usize>,
{
    // Validate range and convert all varians into inclusive byte indexes for start and end
    let r = validate_byterange(s.len(), byte_range);

    let mut accumulate = false;
    let mut res = Vec::new();
    for g in Glyph2::glyph2_indices(s) {
        if r.start == *g.byte_range.start() {
            accumulate = true;
        };

        if r.start > *g.byte_range.start() && r.start <= *g.byte_range.end() {
            // Similar panic message when we try to slice a str in the middle of multibyte UTF-8 character
            panic!(
                "Range.start {} is not a glyph boundary; it is inside '{}' (bytes {}..={})",
                r.start,
                &s[g.byte_range.clone()],
                *g.byte_range.start(),
                *g.byte_range.end()
            );
        }

        if accumulate {
            let e = *g.byte_range.end();
            res.push(g);
            if r.end == e + 1 {
                return res;
            }
        }
    }
    panic!("Internal error, see https://xkcd.com/2200/"); // Should bever get here actually
}

// ------------------------
// get byte iterator

pub fn get_byteiterator_from_byterange<'a, R>(s: &'a str, byte_range: R) -> impl Iterator<Item = u8> + 'a where R: RangeBounds<usize>, {
    s[validate_byterange(s.len(), byte_range)].bytes()
}

// ------------------------
// get char iterator

pub fn get_chariterator_from_byterange<'a, R>(s: &'a str, byte_range: R) -> impl Iterator<Item = char> + 'a where R: RangeBounds<usize>, {
    s[validate_byterange(s.len(), byte_range)].chars()
}

// ------------------------
// get glyph iterator

pub fn get_glyphiterator_from_byterange<R>(s: &str, byte_range: R) -> impl Iterator<Item = Glyph2> where R: RangeBounds<usize>, {
    get_glyphvector_from_byterange(s, byte_range).into_iter()
}

// ------------------------
// get &str

pub fn get_strref_from_byterange<'a, R>(s: &'a str, byte_range: R) -> &'a str 
where
    R: RangeBounds<usize>,
{
    &s[validate_byterange(s.len(), byte_range)]
}

// ------------------------
// get String

pub fn get_string_from_byterange<R>(s: &str, byte_range: R) -> String
where
    R: RangeBounds<usize>,
{
    s[validate_byterange(s.len(), byte_range)].to_string()
}
