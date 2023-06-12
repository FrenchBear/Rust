// l08_collections
// Learning Rust again, arrays, collections, iterators...
//
// 2023-06-11   PV

#![allow(dead_code, unused_variables)]

fn main() {
    arrays();
    my_list();
    vectors();
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
    v:Vec<i32>,
}

impl List {
    fn new() -> List {
        List {
            v: Vec::new()
        }
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

    v2.sort_by(|a, b| b.cmp(a));    // Reverse order    https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust

    use std::cmp::Reverse;
    v2.sort_by_key(|w| Reverse(*w));    // Reverse order too

    v2.sort();  // Sort then reverse, but not always equivalent to reverse sort using keys comparison, and not stable
    v2.reverse();

    v2.remove(1);   // Delete element using index

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
