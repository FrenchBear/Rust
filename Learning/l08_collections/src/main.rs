// l08_collections
// Learning Rust again, arrays, collections, iterators...
//
// 2023-06-11   PV

#![allow(dead_code, unused_variables)]

fn main() {
    arrays();
    my_list();
    vectors();
    strings();
}

fn arrays() {
    // Arrays (init: https://www.joshmcguigan.com/blog/array-initialization-rust/)
    let mut a1: [i32; 10] = [0; 10]; // Initialization from [T; N] where T: Copy
    let a1b: [i32; 10] = Default::default(); // Initialization from [T; N] where T: Default (and N <= 32)
    let a2 = [1, 2, 3];
    let a3 = ["Once", "upon", "a", "time"];
    a1[0] = 5;
    a1[1] = 6;
    a1[2] = 7;

    for i in &a2 {
        print!("{i} ")
    }
    println!("\n");
}

// -------------------------------------------------------------
// Encapsulation of a vector in a struct

fn my_list() {
    let mut l = List::new();
    l.add(10);
    l.add(20);
    l.add(30);
    for i in &l.v {
        print!("{i} ")
    }
    println!("\n");
}

struct List {
    v: Vec<i32>,
}

impl List {
    fn new() -> List {
        List { v: Vec::new() }
    }

    fn add(&mut self, value: i32) {
        self.v.push(value);
    }
}

// -------------------------------------------------------------

fn vectors() {
    // Vectors
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    let v3 = vec!["Once", "upon", "a", "time"];
    v1.push(5);
    v1.push(6);
    v1.push(7);
    let third = v1[2]; // Ok since integers support copy
    v1[2] = -3;
    println!("Third: {third}");
    let third: &i32 = &v1[2];
    println!("Third: {third}");
    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("Third: {third}"),
        None => println!("No third."),
    }
    let third: Option<i32> = v1.get(2).copied(); // copied: Maps an Option<&T> to an Option<T> by copying the contents of the option
    match third {
        Some(third) => println!("Third: {third}"),
        None => println!("No third."),
    }
    println!();

    let vs: Vec<String> = vec![String::from("Hello"), String::from("world")];
    // let s = vs[1];  // move occurs because value has type `String`, which does not implement the `Copy` trait
    let s = &vs[1]; // Ok using a reference
    println!("s={s}");

    for s in &vs {
        print!("{s} ");
    }
    println!();

    // Iterating over values and changing them
    for i in &mut v2 {
        *i = 2 * (*i);
    }

    // Sort vector
    v2.sort();

    v2.sort_by(|a, b| b.cmp(a)); // Reverse order    https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust

    use std::cmp::Reverse;
    v2.sort_by_key(|w| Reverse(*w)); // Reverse order too

    v2.sort(); // Sort then reverse, but not always equivalent to reverse sort using keys comparison, and not stable
    v2.reverse();

    v2.remove(1); // Delete element using index

    // References in Rust are handled differently than references in C++ or C#
    let mut n = 5;
    let an = &mut n;
    *an = 6; // Use of an &i32 requires * to access the value (contrary to C++ or C# references)
    println!("an={an}"); // But printing does not, "an={*an}" causes an error... {} in println is only for variable+format, not expression

    // Multiple types in a Vector using an Enum (can also use a trait)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!();
    for v in &row {
        match v {
            SpreadsheetCell::Int(i) => println!("Int {i}"),
            SpreadsheetCell::Float(f) => println!("Float {f}"),
            SpreadsheetCell::Text(s) => println!("Text {s}"),
        }
    }
    println!();

    // Vector of vectors
    let mut vv: Vec<Vec<i32>> = Vec::new();
    vv.push(Vec::new());
    vv.push(Vec::new());
    vv.push(Vec::new());
}

fn strings() {
    println!();

    let mut s = String::from("foo");
    s.push_str("bar"); // push_str use a str& and does not take ownership of parameter
    s.push('!'); // add single char

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // Although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result.
                       // Signature of + operator: (it's self, not &self):  fn add(self, s: &str) -> String {

    let s4 = s2.clone() + &s3;
    //println!("s1={s1}");  // s1 not valid
    println!("s2={s2}");
    println!("s3={s3}");
    println!("s4={s4}");

    // Format works like println! and does not take ownership of parameters
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // tic-tac-toe

    // -----------
    println!();
    let s = "Aéà♫山𝄞🐗"; // à is decomposed form (combining accent and a)
    println!("{}", s);

    let bs = s.as_bytes();
    let mut i=0;
    while i<s.len() {
        let b = bs[i];
        if b<128 {
            if let Ok(c) = String::from_utf8(bs[i..=i].to_vec()) {
                print!("[{b:02x}: {c}]");
            }
            i+=1;
        } else if b&0b11100000 == 0b11000000 {
            let b2 = bs[i+1];
            if let Ok(c) = String::from_utf8(bs[i..=i+1].to_vec()) {
                print!("[{b:02x} {b2:02x}: {c}]");
            }
            i+=2;
        } else if b&0b11110000 == 0b11100000 {
            let b2 = bs[i+1];
            let b3 = bs[i+2];
            if let Ok(c) = String::from_utf8(bs[i..=i+2].to_vec()) {
                print!("[{b:02x} {b2:02x} {b3:02x}: {c}]");
            }
            i+=3;
        } else if b&0b11111000 == 0b11110000 {
            let b2 = bs[i+1];
            let b3 = bs[i+2];
            let b4 = bs[i+3];
            if let Ok(c) = String::from_utf8(bs[i..=i+3].to_vec()) {
                print!("[{b:02x} {b2:02x} {b3:02x} {b4:02x}: {c}]");
            }
            i+=4;
        } else {
            panic!("Error");
        }
    }
    println!();
    
}
