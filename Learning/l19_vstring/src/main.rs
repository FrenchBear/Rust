// l19_string
// Learning rust 2024, Prepare a module containing string helpers
//
// 2024-12-13   PV
//
// Two strings in rust:
// - str:    in the core language, usually seen in its borrowed form &str. String slices are references to some UTF-8 encoded string data stored elsewhere.
//           String literals, for example, are stored in the program’s binary and are therefore string slices.
// - String: Provided by Rust’s standard library is a growable, mutable, owned, UTF-8 encoded string type.
//           Many of the same operations available with Vec<T> are available with String as well because String is actually implemented as a wrapper
//           around a vector of bytes Vec<u8> with some extra guarantees, restrictions, and capabilities.

#![allow(dead_code, unused_variables, unused_imports)]

mod glyph2;
mod tests;
mod vstring;

use glyph2::Glyph2;
use vstring::*;

fn main() {
    test_vstrings();

    let s1 = get_byteslice_from_byterange("ABC", ..);
    let v1 = get_bytevector_from_byterange("ABC", ..);
}

pub fn test_vstrings() {
    // let s = "Aé♫山𝄞🐗🐻‍❄️";
    // println!("s={s}");

    // println!("\nBytes functions");
    // println!("get_byte_length={}", get_byte_length(s));
    // println!("get_char_length={}", get_char_length(s));
    // println!("get_glyph_length={}", get_glyph_length(s));
    // println!();

    let s = "👨‍❤‍👩e\u{0301}🐻‍❄️";

    for c in get_chariterator_from_glyphrange(s, 2..) {
        println!("assert_eq!(it.next(), Some('{}'));", c);
    }
}
