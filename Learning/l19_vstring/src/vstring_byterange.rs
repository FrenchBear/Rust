// vstrings library - byterange based functions
// Learning rust 2024, A bunch of string helpers before working on dev for fun project String coding
//
// 2024-11-10   PV
// 2024-12-13   PV      Separated module, more functions
//

#![allow(unused_mut)]

use core::ops::{Range, RangeInclusive};
use std::ops::{RangeFrom, RangeFull, RangeTo, RangeToInclusive};
use std::str;

// ==========================================================================================
// From byterange

// ------------------------
// get byte slice

// Basic version, no range
pub fn get_byteslice<'a>(s: &'a str) -> &'a [u8] {
    s.as_bytes()
}

// Explore variants to return results in case of errors: basic version that panics, Option<&[u8]>, Result<&[u8], String>,
// and a tolerant version that always return a &[u8]. This will not be done for all other cases

// Simple implementation, panicks if range is invalid or goes beyond s limits
pub fn get_byteslice_from_byterange<'a>(s: &'a str, byterange: Range<usize>) -> &'a [u8] {
    &s[byterange].as_bytes()
}

// Option<&[u8]> variant, returns None in cases causing basic version to panic, and Some<&[u8]> if Ok
pub fn get_bytesliceoption_from_byterange<'a>(s: &'a str, byterange: Range<usize>) -> Option<&'a [u8]> {
    if byterange.start > byterange.end || byterange.start > s.len() || byterange.end > s.len() {
        return None;
    }
    Some(&s[byterange].as_bytes())
}

// Result<&[u8],String> varant, return an error string in cases causing basic version to panic, ok Ok(&[u8])
pub fn get_bytesliceresult_from_byterange<'a>(s: &'a str, byterange: Range<usize>) -> Result<&'a [u8], String> {
    if byterange.start > byterange.end {
        return Err(format!("Invalid range, start {} is greater than end {}", byterange.start, byterange.end));
    }
    if byterange.start >= s.len() {
        return Err(format!("Range.start {} is too large for s.len {}", byterange.start, s.len()));
    }
    if byterange.end > s.len() {
        return Err(format!("Range.end {} is too large for s.len {}", byterange.end, s.len()));
    }
    Ok(&s[byterange].as_bytes())
}

// Tolerant version, in invalid cases or range beyond s limits, return empty byte slice or limit range to actual s bounds
pub fn get_byteslicetolerant_from_byterange<'a>(s: &'a str, byterange: Range<usize>) -> &'a [u8] {
    // Invalid range or start after s end, return an empty byte slice
    if byterange.start >= byterange.end || byterange.start >= s.len() {
        return b"";
    }

    // Ensure that the actual range end is clipped to s.len()
    let en = if byterange.end > s.len() { s.len() } else { byterange.end };
    &s[byterange.start..en].as_bytes()
}

// Explore all range variants here, won't be detailed for other returns such as Vec<u8>, Iterator<char>, ...

// Variant with a range inclusive: start..=end
pub fn get_byteslice_from_byterangeinclusive<'a>(s: &'a str, byterange: RangeInclusive<usize>) -> &'a [u8] {
    &s.as_bytes()[byterange]
}

// Variant, range from: start..
pub fn get_byteslice_from_byterangefrom<'a>(s: &'a str, byterange: RangeFrom<usize>) -> &'a [u8] {
    &s.as_bytes()[byterange]
}

// Variant, range to: ..end
pub fn get_byteslice_from_byterangeto<'a>(s: &'a str, byterange: RangeTo<usize>) -> &'a [u8] {
    &s.as_bytes()[byterange]
}

// Variant, range to inclusive: ..=end
pub fn get_byteslice_from_byterangetoinclusive<'a>(s: &'a str, byterange: RangeToInclusive<usize>) -> &'a [u8] {
    &s.as_bytes()[byterange]
}

// Variant with a full range = ..
// This seems useless
pub fn get_byteslice_from_bytefullrange<'a>(s: &'a str, _: RangeFull) -> &'a [u8] {
    &s.as_bytes()[..]
}

pub fn get_byteslice_from_startbytecount<'a>(s: &'a str, bytecount: usize) -> &'a [u8] {
    &s.as_bytes()[0..bytecount]
}

pub fn get_byteslice_from_endbytecount<'a>(s: &'a str, bytecount: usize) -> &'a [u8] {
    &s.as_bytes()[s.len() - bytecount..]
}

// ------------------------
// get byte vector, copying bytes

// Basic version, no range
pub fn get_bytevector(s: &str) -> Vec<u8> {
    s.bytes().collect()
    // Vec::from(s.as_bytes())
}

// Returning a Vec<u8> is Ok, but it duplicates characters
pub fn get_bytevector_from_byterange(s: &str, byterange: Range<usize>) -> Vec<u8> {
    // ToDo: Check which version is the most efficient
    //Vec::from(&s.as_bytes()[byterange])
    s[byterange].bytes().collect()
}

pub fn get_bytevector_from_byterangeinclusive(s: &str, byterange: RangeInclusive<usize>) -> Vec<u8> {
    Vec::from(&s.as_bytes()[byterange])
    // s[byterange].bytes().collect()
}

// and many range variants

// ----------------------------------
// get byte iterator

// Basic version, no range
pub fn get_byteiterator<'a>(s: &'a str) -> impl Iterator<Item = u8> + 'a {
    s.bytes()
}

// Returning an iterator on bytes
pub fn get_byteiterator_from_byterange<'a>(s: &'a str, byterange: Range<usize>) -> impl Iterator<Item = u8> + 'a {
    s[byterange].bytes()
}

pub fn get_byteiterator_from_byterangeinclusive<'a>(s: &'a str, byterange: RangeInclusive<usize>) -> impl Iterator<Item = u8> + 'a {
    s[byterange].bytes()
}

// and many range variants
