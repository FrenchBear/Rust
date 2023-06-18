// l08_collections
// Learning Rust again, arrays, collections, iterators...
//

// 2023-06-11   PV

#![allow(dead_code, unused_variables)]

extern crate unicode_normalization;

use unicode_categories::UnicodeCategories;
use unicode_normalization::UnicodeNormalization;

fn main() {
    arrays();
    my_list();
    vectors();
    strings();
    hashmaps();
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

    let t1: [f64; 3] = [1.0, 5.0, 6.0];
    let m1 = median(&t1).unwrap_or(-999.0);
    println!("Median1: {m1}");
    let t2: [f64; 4] = [1.0, 5.0, 6.0, 7.0];
    let m2 = median(&t2).unwrap_or(-999.0);
    println!("Median2: {m2}");

    let v1 = vec![1, 2, 1, 6, 3, 4, 2, 2, 3, 5, 1, 5, 3, 4, 2, 2, 6, 1, 4];
    let m = mode(&v1);
    println!("Mode {m}");
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in v {
        let cnt = map.entry(*i).or_insert(0);
        *cnt += 1;
    }
    let mut imax = 0;
    let mut cntmax = 0;
    for (k, v) in map {
        if v > cntmax {
            cntmax = v;
            imax = k;
        }
    }

    imax
}

// Exercice 10.4
fn median(tf: &[f64]) -> Option<f64> {
    let mut vf: Vec<f64> = Vec::new();
    for f in tf {
        vf.push(*f);
    }
    vf.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let l = vf.len();
    if l & 1 == 1 {
        return vf.get(l / 2).copied();
    }
    if l == 0 {
        return None;
    }
    let f1 = vf.get(l / 2 - 1).copied().unwrap_or(0.0);
    let f2 = vf.get(l / 2).copied().unwrap_or(0.0);
    Some((f1 + f2) / 2.0)
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
    // Print individual characters with associated bytes in a UTF-8 string
    println!();
    let s = "Aéà♫山𝄞🐗"; // à is decomposed form (combining accent and a)
    println!("{}", s);

    let bs = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        let b = bs[i];
        if b < 128 {
            if let Ok(c) = String::from_utf8(bs[i..=i].to_vec()) {
                print!("[{b:02x}: {c}]");
            }
            i += 1;
        } else if b & 0b11100000 == 0b11000000 {
            let b2 = bs[i + 1];
            if let Ok(c) = String::from_utf8(bs[i..=i + 1].to_vec()) {
                print!("[{b:02x} {b2:02x}: {c}]");
            }
            i += 2;
        } else if b & 0b11110000 == 0b11100000 {
            let b2 = bs[i + 1];
            let b3 = bs[i + 2];
            if let Ok(c) = String::from_utf8(bs[i..=i + 2].to_vec()) {
                print!("[{b:02x} {b2:02x} {b3:02x}: {c}]");
            }
            i += 3;
        } else if b & 0b11111000 == 0b11110000 {
            let b2 = bs[i + 1];
            let b3 = bs[i + 2];
            let b4 = bs[i + 3];
            if let Ok(c) = String::from_utf8(bs[i..=i + 3].to_vec()) {
                print!("[{b:02x} {b2:02x} {b3:02x} {b4:02x}: {c}]");
            }
            i += 4;
        } else {
            panic!("Error");
        }
    }
    println!();

    // -----------
    // Conversions String <=> Vec<Char>
    println!();
    let s = String::from("Aé♫山𝄞🐗🐻‍❄");
    let v: Vec<char> = s.chars().collect();
    let t: String = v.iter().collect();
    assert_eq!(s, t);

    // -----------
    // Remove Mn category (Mark non-spacing = combining accents) characters
    println!();
    let s = "Élément où ça? Là!";
    let s = &s.nfd().collect::<String>()[..];
    let mut r = String::new();
    for c in s.chars() {
        let cv = c as u32;
        let l = c.is_letter();
        let m = c.is_mark_nonspacing();
        //print!("{cv:04X} {c}: l={l} m={m}  ");
        if !m {
            r.push(c);
        }
    }
    println!("{s}\n{r}");

    // -----------
    // Exercise 10.4
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    println!();
    let s = pig_latin("first apple");
    println!("{s}")
}

fn pig_latin(s: &str) -> String {
    let mut vs: Vec<String> = Vec::new();
    for word in s.split_whitespace() {
        let vw: Vec<char> = word.chars().collect();
        //let c1 = word.chars().next().unwrap();
        let c1 = vw[0];
        if c1 == 'a' || c1 == 'e' || c1 == 'i' || c1 == '0' || c1 == 'u' || c1 == 'y' {
            vs.push(String::from(word) + "hay");
        } else {
            let nw: String = vw[1..].iter().collect();
            vs.push(format!("{nw}{c1}ay"));
        }
    }
    vs.join(" ")
}

use std::collections::HashMap;

fn hashmaps() {
    println!();
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 12); // Inserting again updates value
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Yellow")).or_insert(100); // Insert only if the key doesn' already have a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For owned values like String, invert moves the values and the hashmap is the owner
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{field_name}");     // Error
    //println!("{field_value}");    // Error

    // Inserting a reference won't move values, but the values referenced must be valid at least as long as the hashmap is valid
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{field_name}");
    println!("{field_value}");

    // For chars, a copy is made automatically
    let field_name = 'f';
    let field_value = '𝄞';
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{field_name}");
    println!("{field_value}");

    // counter
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
