// r20_smart_pointers
// Learning rust 2024
//
// 2025-01-09   PV

#![allow(dead_code, unused_variables)]

use std::time::Instant;

// A recursive type, without box, compiler wouldn't know the size of List
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Variant using a ref
enum List2<'a> {
    Cons2(i32, &'a List2<'a>),
    Nil2,
}

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};

fn main() {
    // Simple Box, storing data on the heap (but the box itself, ie. the pointer, is stored on the stack)
    let b = Box::new(5);
    println!("b={b}");

    // Use a recursive type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Note that a ref also works
    let list2 = Cons2(1, &Cons2(2, &Cons2(3, &Nil2)));

    // Contrary to what is repeated in the Book, of course, boxing/unboxing has a penaly cost
    // And contrary to .Net where boxing/unboxing is invisible, in Rust, unboxing is explicit with dereferencement operator *
    test_box_performace();


}

fn test_box_performace() {
    const LOOPS: i32 = 10_000_000;
    let start = Instant::now();

    let mut a: i32 = 0;
    let mut b: i32;
    for _ in 0..LOOPS {
        a += 1;
        b = a + 1;
        let c = b + a;
    }
    let duration = start.elapsed().as_millis();
    println!("test stack: {duration}ms");
    // 45ms

    let mut a: Box<i32> = Box::new(0);
    let mut b: Box<i32>;
    for _ in 0..LOOPS {
        a = Box::new(*a + 1);
        b = Box::new(*a + 1);
        let c = Box::new(*b + *a);
    }
    let duration = start.elapsed().as_millis();
    println!("test heap: {duration}ms");
    // 1231 ms
}
