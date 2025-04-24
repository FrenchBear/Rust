// r21_rc
// Learning rust 2024, Smart Pointers 2
//
// 2025-01-10   PV

#![allow(dead_code, unused_variables)]

// When multiple ownership is needed, Rc(Reference Counting) can be used. Rc keeps track of the number of the references
// which means the number of owners of the value wrapped inside an Rc.
//
// Reference count of an Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever one cloned Rc is
// dropped out of the scope. When an Rc's reference count becomes zero (which means there are no remaining owners), both
// the Rc and the value are all dropped.
//
// Cloning an Rc never performs a deep copy. Cloning creates just another pointer to the wrapped value, and increments
// the count.


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
    println!("a={:?} ref count={}\n\n", ra, Rc::strong_count(&ra));


    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        let rc_a: Rc<String> = Rc::new(rc_examples);        // Note that value is moved and not directly available after that

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("--- rc_a is cloned to rc_b ---");
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            println!("--- rc_b is dropped out of scope ---");
        }
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
    
    println!("---");

    let ref_examples = "ref examples".to_string();
    {
        println!("--- ref_a is created ---");
        let ref_a = &ref_examples;        // Note that value is moved and not directly available after that
        {
            let ref_b = ref_a;
            println!("ref_a and ref_b are equal: {}", ref_a.eq(ref_b));
            println!("Length of the value inside ref_a: {}", ref_a.len());
            println!("Value of ref_b: {}", ref_b);
            println!("--- ref_b is dropped out of scope ---");
        }
        println!("--- ref_a is dropped out of scope ---");
    }

    let r2a = get_ref_2();
    let r2b = r2a.clone();

    let mut r3a = get_ref_3();
    let r3b = r3a.clone();          // Cloning a box clones its contents

    r3a.push('?');
    println!("r3a: {r3a}");
    println!("r3b: {r3b}");

}


// Not allowed to return a reference to a local variable
// fn get_ref_1() -> &str {
//     let s = String::from("Hello");
//     &s
// }

fn get_ref_2() -> Rc<String> {
    let s = String::from("Hello");
    Rc::new(s)
}

fn get_ref_3() -> Box<String> {
    let s = String::from("Hello");
    Box::new(s)
}
