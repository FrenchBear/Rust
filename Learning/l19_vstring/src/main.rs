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

mod vstring;
mod glyph2;
mod tests;

use glyph2::Glyph2;
use vstring::*;

fn main() {
    test_vstrings();
}

pub fn test_vstrings() {

    let s = "Aé♫山𝄞🐗🐻‍❄️";
    println!("s={s}");

    println!("\nBytes functions");
    println!("get_byte_length={}", get_byte_length(s));
    println!("get_char_length={}", get_char_length(s));
    println!("get_glyph_length={}", get_glyph_length(s));
    println!();

    let s = "Aé♫山𝄞🐗";
    for b in get_byteiterator_from_charrange(s, 2..4){
        println!("{:#02X}", b);        
    }
    println!();

    // for g in get_glyphvector_from_charrange("👨‍❤‍👩e\u{0301}🐻‍❄️", 6..7) {
    //     print!("{:?}, ", g)
    // }
    // println!();
}
