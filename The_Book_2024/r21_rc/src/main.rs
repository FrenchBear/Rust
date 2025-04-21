// r21_rc
// Learning rust 2024, Smart Pointers 2
//
// 2025-01-10   PV

#![allow(dead_code, unused_variables)]

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl Drop for List {
    fn drop(&mut self) {
        println!("drop called on struct List instance");
    }
}

use crate::List::{Cons, Nil};

fn main() {
    // Rc<T>, reference counted
    // Can create clones, content is dropped once last clone is dropped
    // Allow only immutable borrows
    let ra = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&ra));
    {
        let c = Cons(4, Rc::clone(&ra));
        println!("a={:?} ref count={}", ra, Rc::strong_count(&ra));
    }
    println!("a={:?} ref count={}", ra, Rc::strong_count(&ra));
}
