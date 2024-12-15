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

#![allow(dead_code, unused_variables)]

mod vstring;
mod glyph2;
mod tests;

use vstring::*;

fn main() {
    test_vstrings();
}

pub fn test_vstrings() {

    let s = "Aé♫山𝄞🐗";
    println!("s={s}");

    // Bytes
    println!("\nBytes functions");
    println!("get_byte_length={}", get_byte_length(s));
    println!("get_byte_from_index(10)={}", get_byte_from_byteindex(s, 10));
    println!("get_byteoption_from_index(20)={:?}", get_byteoption_from_byteindex(s, 20));
    println!("get_str_from_byteslice(b\"Hello\")={:?}", get_strref_from_byteslice(b"Hello"));
    
    println!("get_byteslice_from_range(1..3)={:?}", get_byteslice_from_byterange(s, 1..3));
    println!("get_byteslice_from_range(0..10)={:?}", get_byteslice_from_byterange(s, 0..10));
    println!("get_byteslice_from_range(0..=9)={:?}", get_byteslice_from_byterange(s, 0..=9));
    println!("get_byteslice_from_range(..)={:?}", get_byteslice_from_byterange(s, ..));
    println!("get_byteslice_from_start(5)={:?}", get_byteslice_from_startbytecount(s, 5));
    println!("get_byteslice_from_end(5)={:?}", get_byteslice_from_endbytecount(s, 5));

    println!("get_byteiterator_from_range(3..5)={:?}", get_byteiterator_from_byterange(s, 3usize..5usize).collect::<Vec<u8>>());
    //println!("get_byterefiterator_from_range(3..5)={:?}", get_byterefiterator_from_range(s, &(3usize..5usize)).collect::<Vec<&u8>>());

    /*
    println!("\nChar functions");
    println!("clen={}", get_char_length(s));
    println!("cgetchar(5)={}", get_char_from_index(s, 5));
    println!("cgetcharopt(6)={:?}", get_charoption_from_index(s, 6));
    println!("cgetcharange(2..5)={:?}", get_charslice_from_range(s, &(2usize..5usize)));
    println!("cgetcharangeinclusive(2..=5)={:?}", cgetcharangeinclusive(s, &(2usize..=5usize)));

    let s = "🏳️‍🌈🐻‍❄️";
    println!("\ns={s}");
    println!("blen={}", get_byte_length(s));
    println!("clen={}", get_char_length(s));
     */

}
