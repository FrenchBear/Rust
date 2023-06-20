// l12_lifetimes
// Learning Rust again
//
// 2023-06-20   PV

#![allow(unused)]

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// The signature  express the following constraint: the returned reference will be valid as long as both the parameters are valid.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
