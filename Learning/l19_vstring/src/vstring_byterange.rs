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
// get_byteslice

// Basic version, no range
pub fn get_byteslice<'a>(s: &'a str) -> &'a [u8] {
    &s.as_bytes()[..]
}

pub fn get_byteslice_from_byterange<'a>(s: &'a str, byterange: Range<usize>) -> &'a [u8] {
    &s.as_bytes()[byterange]
}

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
// get_bytevector, copying bytes

// Basic version, no range
pub fn get_bytevector(s: &str) -> Vec<u8> {
    Vec::from(s.as_bytes())
}

// Returning a Vec<u8> is Ok, but it duplicates characters
pub fn get_bytevector_from_byterange(s: &str, byterange: &Range<usize>) -> Vec<u8> {
    Vec::from(&s.as_bytes()[byterange.clone()])
}

pub fn get_bytevector_from_byterangeinclusive(s: &str, byterange: &RangeInclusive<usize>) -> Vec<u8> {
    Vec::from(&s.as_bytes()[byterange.clone()])
}

// can add many variants

// ----------------------------------
// get byteiterator

// Basic version, no range
pub fn get_byteiterator<'a>(s: &'a str) -> impl Iterator<Item = u8> + 'a {
    s.bytes()
}

// Returning an iterator on bytes
pub fn get_byteiterator_from_byterange<'a>(s: &'a str, byterange: &Range<usize>) -> impl Iterator<Item = u8> + 'a {
    s.as_bytes()[byterange.clone()].iter().copied()
}

pub fn get_byteiterator_from_byterangeinclusive<'a>(s: &'a str, byterange: &RangeInclusive<usize>) -> impl Iterator<Item = u8> + 'a {
    s.as_bytes()[byterange.clone()].iter().copied()
}

// and many variants

