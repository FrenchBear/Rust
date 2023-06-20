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

#![allow(unused)]

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let fw = first_word("Il était un petit navire");
    println!("First word: {fw}");

    moby();
}

// The signature  express the following constraint: the returned reference will be valid as long as both the parameters are valid.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let ts = s.split_whitespace();
    let fw = ts.into_iter().next();

    match (fw) {
        Some(w) => w,
        None => "",
    }
}

// ----------------

struct ImportantExcerpt<'a> {
    part: &'a str,
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
