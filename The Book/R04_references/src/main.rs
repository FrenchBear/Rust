// R04_references
// Learning Rust
// 2018-10-19	PV

fn main() {
    let mut s = String::from("Hello");
    {
        let r1 = &s;
        print_len(r1);
    }
    s.push_str(" world");
    print_len(&s);

    // Without this block, can't call first_word_index with a non-mutable reference later
    {
        let r1 = &mut s;
        r1.push_str("!");
        print_len(r1);
    }

    println!("first_word_index: {}", first_word_index(&s));

    let w = &s[6..=10];
    println!("w: {}", w);

    println!("first_word: {}", first_word(&s));

    let mut s2 = String::from(&s[..]);
    s2.clear();
    println!("s2: {}", s2);
}

fn print_len(s: &String) {
    println!("Len of {} is {}", s, s.len());
}

//#[allow(dead_code)]
fn first_word_index(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
