// r15_lifetimes
// Learning rust 2024, The Book §10, 10 Generic Types, Traits, and Lifetimes
//
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions
//
// 2024-11-22   PV
// 2025-04-21   PV      Clippy suggestions

#![allow(dead_code, unused_variables)]

use std::fmt::Display;

// This means that the lifetime returned reference is the same as the smaller of the values referenced by function arguments
// Annotations go into function signature, not int he function body
// Specifying the lifetime does not change the lifetime of anu values passed or returned
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn constant_str<'b>(x: &str, y: &str) -> &'b str {
    "Hello"
}

fn test_lonngest_string() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    // The cocnrete lifetime substituted for 'a is the smaller of lifetimes of x and y, and the
    // returned reference will also be valid for the same smaller lifetime
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let a = String::from("a");
    let b = "b".to_string();
    let c = constant_str(&a[..], b.as_str());
    println!("c={}", c)
}

// ------------------------------------------------------------------------
// A struct containing a reference must have a lifetime annotation
// This means that an instance of ImportantExceprpt cannot outlive the reference it holds in part
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Since struct as a lafetime annotation, impl MUST refer to this annotation, even in this case where
// leven doesn't return a reference.  Note that the alision rule apply to &self, so it's actually &'a self
impl ImportantExcerpt<'_> {
    fn level(&self) -> i32 {
        3
    }

    // Lifetime elision rules apply:
    // self gets 'a lifetime because it's &self
    // announcement gets 'b lifetime: all args get their own lifetime
    // returned value gets 'a lifetime because of the reference to self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// ------------------------------------------------------------------------
// Lifetime elision example, built-in rule in rust compiler
// Equivalent to fn first_word<'a>(s: &'a str) -> &'a str {
// Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    //&s[..]
    s
}

// ------------------------------------------------------------------------
// Generic Type Parameters, Trait Bounds, and Lifetimes Together

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

// ------------------------------------------------------------------------

fn main() {
    test_lonngest_string();

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
