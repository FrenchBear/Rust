// l12_lifetimes
// Learning Rust again
//
// When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.
// The lifetime annotations become part of the contract of the function, much like the types in the signature.
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
// Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers
// or otherwise violate memory safety.
//
// 2023-06-20   PV
// 2025-04-21   PV      Clippy optimizations

#![allow(unused)]

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let fw = first_word("Il était un petit navire");
    println!("First word: {fw}");

    moby();
    irtest();

    // static lifetime can actually be used (while 'a can't outside of a function with <'a>), but it's implicit here, all string
    // literals have 'static lifetime
    let s: &'static str = "I have a static lifetime.";
}

// The signature  express the following constraint: the returned reference will be valid as long as both the parameters are valid.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Actually the definition of first_word should be:
// fn first_word<'a>(s: &'a str) -> &'a str {
// But thanks to lifetime elision rules, the compiler can infer this definition from the one without lifetimes
fn first_word(s: &str) -> &str {
    let ts = s.split_whitespace();
    let fw = ts.into_iter().next();
    fw.unwrap_or_default()
}

// ----------------

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Not allowed, since struct has lifetime, impl must have lifetime (lifetime is part of the type)...
// impl ImportantExcerpt { }

//impl<'a> ImportantExcerpt<'a> {
impl ImportantExcerpt<'_> {
    fn level(&self) -> i32 {
        // No need for a lifetime here for the reference to self
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // Result gets by default the same lifetime as &self, so it's Ok
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn moby() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("First sentence: {}", i.part);
    }
    //println!("First sentence: {}", i.part);
}

// ----------------

struct Iref<'a, 'b> {
    ir1: &'a i32,
    ir2: &'b i32,
}

fn irtest() {
    let v1 = [1, 2, 3];
    let ir;
    let jr;
    {
        let i = 42;
        ir = Iref {
            ir1: &v1[1],
            ir2: &i,
        };

        println!("ir: {} {}", ir.ir1, ir.ir2);
        jr = ir.ir1;
    }
    //let n =  *ir.ir1;     // Doesn't work, seems useless to use two lifetimes in a struct if the compiler only use the most restrictive even if it doesn't apply
    println!("jr: {jr}") // But this works
}

// ----------------

use std::fmt::Display;

// Using both a lifetime and a generic type between < >
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann); // Display trait is required
    if x.len() > y.len() { x } else { y }
}
