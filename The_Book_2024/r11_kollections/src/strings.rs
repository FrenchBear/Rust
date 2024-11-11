// r11_kollections/strings.rs
// Learning rust 2024, The Book §8, common collections
//
// 2024-11-10   PV
//
// Two strinfs in rust:
// - str:    in the core language, usually seen in its borrowed form &str. String slices are references to some UTF-8 encoded string data stored elsewhere.
//           String literals, for example, are stored in the program’s binary and are therefore string slices.
// - String: Provided by Rust’s standard library is a growable, mutable, owned, UTF-8 encoded string type.
//           Many of the same operations available with Vec<T> are available with String as well because String is actually implemented as a wrapper
//           around a vector of bytes Vec<u8> with some extra guarantees, restrictions, and capabilities.

#![allow(dead_code, unused_variables, unused_mut)]

use core::ops::{Range, RangeInclusive};

pub fn test_strings() {
    println!("\ntest_strings");

    // Creating a new, empty string
    let mut s1 = String::new();

    // Create a string from a litteral (&str)
    let data = "initial contents";
    let s2 = data.to_string();

    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();

    let s4 = String::from("initial contents");

    // Appending
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str takes a slice since we usually don't want to take ownership of parameter, so we can print s2 here
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    s.push('😊'); // Contrary to .Net, char cover all Unicode range, utf-8 encoded (no UTF-16/UCS2 as in .Net)

    // Concatenation with +, requires an owned string on the left
    // The + operator uses the add method, whose signature looks like: fn add(self, s: &str) -> String {    self doesn't have a & so it takes ownership
    // We can only add a &str to a string, can't add two String values together
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // to concatenate multiple strings, the behavior of the + operator gets unwieldy:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let sr1 = s1 + "-" + &s2 + "-" + &s3;
    //print!("s1={s1}");      // Invalid

    // format! macro
    // Only use references, does NOT take ownership of the first string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let sr2 = format!("{s1}-{s2}-{s3}");
    println!("s1={s1}\n"); // Valid

    // Indexing
    // Can't index string by integer, need to use range of usize
    let s1 = String::from("hello");
    //let h = s1[0];        // Error

    let s = "Aé♫山𝄞🐗";
    println!("s={s}");

    println!("\nBytes functions");
    println!("blen={}", blen(s));
    println!("bgetbyte(10)={}", bgetbyte(s, 10));
    println!("bgetbyteopt(20)={:?}", bgetbyteopt(s, 20));
    println!("bgetbyterange(0..10)={:?}", bgetbyterange(s, &(0usize..10usize)));
    println!("bgetbyterangeinclusive(0..=10)={:?}", bgetbyterangeinclusive(s, &(0usize..=10usize)));

    println!("\nChar functions");
    println!("clen={}", clen(s));
    println!("cgetchar(5)={}", cgetchar(s, 5));
    println!("cgetcharopt(6)={:?}", cgetcharopt(s, 6));
    println!("cgetcharange(2..5)={:?}", cgetcharange(s, &(2usize..5usize)));
    println!("cgetcharangeinclusive(2..=5)={:?}", cgetcharangeinclusive(s, &(2usize..=5usize)));

}

// Byte functions
fn blen(s: &str) -> usize {
    s.len()
}

fn bgetbyte(s: &str, index: usize) -> u8 {
    s.as_bytes()[index]
}

fn bgetbyteopt(s: &str, index: usize) -> Option<u8> {
    s.bytes().nth(index)
}

fn bgetbyterange<'a>(s: &'a str, range: &Range<usize>) -> &'a [u8] {
    &s.as_bytes()[range.clone()]
}

fn bgetbyterangeinclusive<'a>(s: &'a str, range: &RangeInclusive<usize>) -> &'a [u8] {
    &s.as_bytes()[range.clone()]
}

// Char functions
fn clen(s: &str) -> usize {
    s.chars().count()
}

fn cgetchar(s: &str, index: usize) -> char {
    s.chars().nth(index).unwrap()
}

fn cgetcharopt(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

fn cgetcharange<'a>(s: &'a str, range: &Range<usize>) -> &'a str {
    let start = s.char_indices().nth(range.start).unwrap().0;
    let end  = if range.end==clen(s) {blen(s)} else {s.char_indices().nth(range.end).unwrap().0};
    &s[start..end]
}

fn cgetcharangeinclusive<'a>(s: &'a str, range: &RangeInclusive<usize>) -> &'a str {
    let start = *range.start();
    let end = *range.end()+1;
    let newrange = start..end;
    cgetcharange(s, &newrange)
}
