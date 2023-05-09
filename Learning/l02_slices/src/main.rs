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
}

fn first_space(s: &String) -> usize {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return i;
        }
    }
    return s.len();
}
