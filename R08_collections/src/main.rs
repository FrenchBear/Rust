// R08_collections
// Learning Rust
// 2018-10-29	PV

#![allow(unused_mut)]
#![allow(unused_variables)]

fn main() {
    vectors();
    strings();
    hashmaps();
}

fn vectors() {
    let mut v1: Vec<i32> = Vec::new();  // Using variable type annotation to hint Vec about the type of elements
    let mut v2 = Vec::<i32>::new();     // And not Vec<i32>::new() for some reason
    let mut v3 = vec![1, 2, 3];         // Declare and initialize: no type annotation needed

    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);

    // Indexed access
    let third: &i32 = &v1[2];
    let trois: i32 = v1[2];
    //v1.push(4);       // Not accepted because there is an immutable borrow 2 lines above

    // Slices
    let slice = &v1[2..4];

    // Iterate over mutable references (can't do it on v1 since there is an immutable borrow)
    for i in &mut v3 {
        println!("{}", i);
        *i += 100;
    }

    // get accessor returning Option<&T>, doesn't panic if index does not exist contrary to v1[v_index]
    let v_index = 5;
    match v1.get(v_index) {
        Some(_) => { println!("Reachable element at index: {}", v_index); }
        None => { println!("Unreachable element at index: {}", v_index); }
    }

    // use enums to store more than one type in a vector
    enum Mixed {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Mixed::Int(3),
        Mixed::Text(String::from("blue")),
        Mixed::Float(10.12),
    ];

}

fn strings() {
    // Strings and str are UTF-8 encoded
    // Standard library also provide types OsString, OsStr, CString, and CStr

    // new empty string
    let mut s = String::new();

    // With initial content, both forms are equivalent
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // Appending text to a string
    let mut s = String::from("foo");
    s.push_str("bar");      // Uses a string slice, no ownership of parameter taken

    // Use + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;      // Note s1 has been moved here and can no longer be used
    // That's because the + operator uses the add method, whose signature looks something like this:
    //      fn add(self, s: &str) -> String {
    // We can only add a &str to a String; we can’t add two String values together.
    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.
    // When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]. 

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");   // Since s1 has lost ownership of its content
    let s = format!("{}-{}-{}", s1, s2, s3);


    // 8.2.4 Indexing into Strings
    let s = "Aéà♫山𝄞🐗";       // à is decomposed form (combining accent and a)
    println!("s={}  s.len()={}", s, s.len()); // len() = 17 UTF-8 bytes

    let mut l=0;
    println!("s.chars()");
    for c in s.chars() {
        l+=1;
        print!("{} ", c);
    }
    println!("    l={}", l);

    l=0;
    println!("s.bytes()");
    for b in s.bytes() {
        l+=1;
        print!("{} ", b);
    }
    println!("    l={}", l);
}


use std::collections::HashMap;

fn hashmaps() {
    let mut scores = HashMap::new();            // type is inferred from following lines!
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

}
