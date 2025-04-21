// r02_integer_overflow
// Test functions related to integer overflow
//
// 2024-11-04   PV

fn main() {
    let a: u8 = 150;
    let b: u8 = 150;
    // let c:u8 = a+b;      // Doesn't even compile, causes attempt to compute `150_u8 + 150_u8`, which would overflow

    let c1 = u8::wrapping_add(a, b); // Returns (a + b) mod 2^N, where N is the width of T in bits
    println!("wrapping_add: c1={c1}");

    let c2 = u8::checked_add(a, b);
    match c2 {
        Some(res) => println!("checked_add: Ok, c2={res}"),
        None => println!("checked_add: Overflow"),
    };

    let c3 = u8::saturating_add(a, b); // Saturating at the numeric bounds instead of overflowing
    println!("saturating_add: c3={c3}");
}
