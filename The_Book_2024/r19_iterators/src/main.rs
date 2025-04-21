// r19_iterators
// Learning rust 2024
//
// 2024-12-29   PV
// 2025-04-21   PV      Clippy suggestions

#![allow(dead_code, unused_variables)]

fn simple_iter_over_references() {
    let v = [1, 2, 3, 4];
    for i in v.iter() {}
}

fn main() {
    // simple iterator over references
    let v = [1, 2, 3, 4];
    for i in v.iter() {} // i: &i32

    // iterator over mutable references
    let mut v = [1, 2, 3, 4];
    for i in v.iter_mut() {} // i: &mut i32

    // Iterator taking ownership of v and returns owned values
    let v = [1, 2, 3, 4];
    for i in v.into_iter() {} // i: i32

    // Some methods such as sum consume the iterator
    let v = [1, 2, 3, 4];
    let vi = v.iter();
    let tot: i32 = vi.sum();
    // vi ownership has been lost and is consumed

    // Iterator adaptators produce other iterators
}
