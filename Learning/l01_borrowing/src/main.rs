// l01_borrowing
// Learning Rust again, Testing borrowing
//
// 2023-05-08   PV

#![allow(unused)]

fn main() {
    let s = String::from("Hello, world");
    let l = get_length(&s);
    println!("len={l}");

    let mut w = String::from("Bonjour");
    let rw = &mut w;
    rw.push_str(", ");
    w.push_str("Pierre");
    println!("{w}");
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn dangling_reference() {
    let r; // No type, no value, but the variable exists inn this scope.  It can't be used before it gets a value, though (no concept of initial null value)
    {
        let x = 5;
        r = x; // Ok
        //r = &x; // Error: borrowed value doesn't live long enough
    }
    print!("{r}")
}
