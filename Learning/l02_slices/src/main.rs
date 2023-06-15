// l02_slices
// Learning Rust again, Slices
//
// 2023-05-16   PV

#![allow(dead_code, unused_variables)]

fn main() {
    //test_slices();
    test_loops();
}

fn test_slices() {
    let s = String::from("Once_upon_a_time");
    let b = first_space(&s);
    println!("First space: {b}");
    println!("{s}");
    let mut i = 0;
    while i < b {
        print!(" ");
        i += 1;
    }
    println!("^");

    let mut t = String::from("Bonjour à tous");
    let t2 = t.clone();
    let fw = first_word(&t2);       // Doesn't compile with &t because of next line, must use a cloned (deep copy) version
    t.clear();
    println!("Fist word: {fw}");
}

fn first_space(s: &String) -> usize {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word(s: &String) -> &str {
    /*
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
    */

    let s = "Hello world";
    return &s[0..5];
}

fn test_loops() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    //println!("names: {:?}", names);
    // FIXME ^ Comment out this line

    // iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

}
