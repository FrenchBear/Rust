// r20_box_deref_drop
// Learning rust 2024, Smart Pointers 1
//
// 2025-01-09   PV

#![allow(dead_code, unused_variables)]

use std::ops::Deref;
use std::time::Instant;

// A recursive type, without box, compiler wouldn't know the size of List
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl Drop for List {
    fn drop(&mut self) {
        println!("drop called on struct List instance");
    }
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

    // Force early from of variable
    drop(list);

    // Contrary to what is repeated in the Book, of course, boxing/unboxing has a penaly cost
    // And contrary to .Net where boxing/unboxing is invisible, in Rust, unboxing is explicit with dereferencement operator *
    test_box_performace();

    let x = MyBox::new(5);
    let y = 5;
    assert_eq!(*x, y);

    let n1 = MyBox::new("Pierre");
    hello(&n1); // Deref coercion, calling hello with &MyBox<&str>, converted automatically to &str

    let n2 = MyBox::new(String::from("world"));
    hello(&n2);
    // Two Deref coercions chained to go from &MyBox<String> to &str: First converted to &String calling deref,
    // then standard library Deref on String, converting it into a &str. This is equivalent to:
    hello(&(*n2)[..]);
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

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Simple implementation similar to Box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
