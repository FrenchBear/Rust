// r23_rc_and_refcell
// Learning rust 2024, Smart Pointers 4: Combining Rc<T> and RefCell<T>
// Multiple mutable references
//
// 2025-01-11   PV

#![allow(dead_code)]

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    let p = Person {
        name: String::from("Pierre"),
        age: 58,
    };
    let pc = RefCell::new(p);
    let pcr1 = Rc::new(pc);
    let pcr2 = Rc::clone(&pcr1);

    // Two mutations using separate copies of Rc<>, but note that mutable borrows do not overlap
    pcr1.borrow_mut().name.push(' ');
    pcr2.borrow_mut().name.push_str("Violent");

    println!("p: {:?}", pcr1.borrow())
}
