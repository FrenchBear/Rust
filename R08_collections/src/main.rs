// R08_collections
// Learning Rust, Vectors, strings and hash maps
// 2018-10-29	PV

#![allow(unused_mut)]
#![allow(unused_variables)]

fn main() {
    vectors();
    strings();
    hashmaps();
    testvectop();
    testpiglatin();
}

fn vectors() {
    let mut v1: Vec<i32> = Vec::new(); // Using variable type annotation to hint Vec about the type of elements
    let mut v2 = Vec::<i32>::new(); // And not Vec<i32>::new() for some reason
    let mut v3 = vec![1, 2, 3]; // Declare and initialize: no type annotation needed

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
        Some(_) => {
            println!("Reachable element at index: {}", v_index);
        }
        None => {
            println!("Unreachable element at index: {}", v_index);
        }
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

extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;

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
    s.push_str("bar"); // Uses a string slice, no ownership of parameter taken

    // Use + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
                       // That's because the + operator uses the add method, whose signature looks something like this:
                       //      fn add(self, s: &str) -> String {
                       // We can only add a &str to a String; we can’t add two String values together.
                       // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.
                       // When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic"); // Since s1 has lost ownership of its content
    let s = format!("{}-{}-{}", s1, s2, s3);

    // 8.2.4 Indexing into Strings
    let s = "Aéà♫山𝄞🐗"; // à is decomposed form (combining accent and a)
    println!("s={}  s.len()={}", s, s.len()); // len() = 17 UTF-8 bytes

    let mut l = 0;
    println!("s.chars()");
    for c in s.chars() {
        l += 1;
        print!("{} ", c);
    }
    println!("    l={}", l);

    l = 0;
    println!("s.bytes()");
    for b in s.bytes() {
        l += 1;
        print!("{} ", b);
    }
    println!("    l={}", l);

    let s = "Où ça? Là!";
    println!("Avant décomposition: len={}", s.len());
    let s = &s.nfd().collect::<String>()[..];
    println!("Après décomposition: len={}", s.len());
}

use std::collections::HashMap;

fn hashmaps() {
    let mut scores = HashMap::new(); // type is inferred from following lines!
    let kblue = String::from("Blue");
    scores.insert(kblue, 10); // HashMap is now the owner of kblue
    scores.insert(String::from("Yellow"), 50);
    // let z=kblue;         // Error: value used after move
    // could have used &String, but references must be valid as long as HashMap is valid

    let mut strscores = HashMap::new(); // variant using str instead of String
    let sblue = "Blue";
    strscores.insert(sblue, 10);
    strscores.insert("Yellow", 50);
    strscores.entry("Blue").or_insert(15);

    // retrieve a value from a hashmap
    let val = strscores.get(sblue); // no pb to reuse sblue, val is Some(&i32)
    match val {
        Some(&res) => println!("score {}: {}", sblue, res),
        None => println!("No score for {}", sblue),
    }

    // build a HashMap from the content of two vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // Two forms
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let scores = teams
        .iter()
        .zip(initial_scores.iter())
        .collect::<HashMap<_, _>>();

    print!("{{");
    for (key, value) in &scores {
        print!("{}: {}, ", key, value);
    }
    println!("\x08\x08}}");
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn testvectop() {
    let vi = vec![8, 12, 17, 13, 12, 21, 7, 87];
    println!("vi:   {:?}", vi);
    println!("sum:  {}", sum(&vi));
    println!("avg:  {}", avg(&vi));
    println!("med1: {}", med1(&vi));
    println!("med2: {}", med2(&vi));
    println!("most: {}", most(&vi));
}

// Computes sum of Vec<i32>
fn sum(vi: &Vec<i32>) -> i32 {
    let mut s = 0;
    for i in vi {
        s += i;
    }
    return s;
}

// Computes average of Vec<i32>
fn avg(vi: &Vec<i32>) -> f64 {
    (sum(vi) as f64) / (vi.len() as f64)
}

// Computes vector median using sorting
fn med1(vi: &Vec<i32>) -> f64 {
    let mut vj = vi.clone();
    vj.sort();
    if vj.len() % 2 == 1 {
        vj[vj.len() / 2] as f64
    } else {
        (vj[vj.len() / 2] + vj[vj.len() / 2 - 1]) as f64 / 2.0
    }
}

// Computes vector median using select algorithm
fn med2(vj: &Vec<i32>) -> f64 {
    if vj.len() % 2 == 1 {
        select(vj, vj.len() / 2) as f64
    } else {
        (select(vj, vj.len() / 2) + select(vj, vj.len() / 2 - 1)) as f64 / 2.0
    }
}

// Return kth element of vector vi in O(n) time
// From "more programming pearls", §15
// Basically a quicksort (tail-call recursion implemented as loop) only caring about
// half of the set at each loop, the one that contains index k
fn select(vi: &Vec<i32>, k: usize) -> i32 {
    let mut l = 0;
    let mut u = vi.len() - 1;
    let mut x = vi.clone();
    while l < u {
        swap(&mut x, l, randint(l, u)); // Random select pivot to avoid problems with almost sorted arrays
        let mut m = l;
        let mut i = l + 1;
        while i <= u {
            if x[i] < x[l] {
                m += 1;
                swap(&mut x, m, i);
            }
            i += 1;
        }
        swap(&mut x, l, m); // move pivot to its correct position
                            // continue with "below" or "above" part using the one that contains index k
        if k <= m {
            u = m - 1;
        } else if k >= m {
            l = m + 1;
        }
    }
    return x[k];
}

extern crate rand;
fn randint(min: usize, max: usize) -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    return rng.gen_range(min, max + 1);
}

// swaps two elements of a vector
fn swap(v: &mut Vec<i32>, i: usize, j: usize) {
    let t = v[i];
    v[i] = v[j];
    v[j] = t;
}

// compute most frequently found value of a vector
fn most(vi: &Vec<i32>) -> i32 {
    let mut hm = HashMap::<i32, i32>::new();
    for &item in vi {
        let c = hm.entry(item).or_insert(0);
        *c += 1;
    }
    //println!("most(): hm: {:?}", hm);
    let mut mval = -1;
    let mut mcount = 0;
    for (val, count) in hm {
        if count > mcount {
            mcount = count;
            mval = val;
        }
    }
    mval
}

fn testpiglatin() {
    let s = "first apple";
    println!("{} -> {}", s, piglatin(s));
}

// Convert strings to pig Latin. The first consonant of each word is moved to the end of the word and “ay” is
// added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple”
// becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn piglatin(s: &str) -> String {
    let mut res = String::from("");
    for word in s.split_whitespace() {
        if res.len() > 0 {
            res.push(' ');
        }
        let firstletter = word.chars().next().unwrap();
        if isvowel(firstletter) {
            res += &word;
            res += "-hay";
        } else {
            res += &word[1..];
            res += &format!("-{}ay", firstletter);
        }
    }
    return res;
}

fn isvowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y'
}
