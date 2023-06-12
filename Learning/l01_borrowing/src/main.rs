// l01_borrowing
// Learning Rust again, Testing borrowing
//
// 2023-05-08   PV

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
