fn main() {
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
    let fw = first_word(&t);
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
