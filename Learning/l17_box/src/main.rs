// l17_box
// Learning Rust again
//
// 2023-06-29   PV

#![allow(unused, non_snake_case)]

// A recursive type (cons List of Lisp)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Need Box<> to ensure that List has a fixed size
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    simple_box();
    test_mybox();
    test_drop();
}

fn simple_box() {
    // When a Box<T> value goes out of scope, heap data the box is pointing to is cleaned up too
    let b = Box::new(5); // Store an i32 value on the heap
    println!("b={b}");
    let i = *b; // Box implement Deref trait

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}"); // Cons(1, Cons(2, Cons(3, Nil)))

    // Using a box like a reference
    let mut x = 5;
    let y = Box::new(x); // We box a copied value of x
    x += 1;
    assert_eq!(6, x);
    assert_eq!(5, *y);
}

// ---------------------

// Defining our own smart pointer (Note: value is not stored on the heap, that doesn't matter here)
struct MyBox<T>(T);

// Without the Deref trait, the compiler can only dereference & references. The deref method gives the compiler the ability to take a value of any type
// that implements Deref and call the deref method to get a & reference that it knows how to dereference.
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    // The type Target = T; syntax defines an associated type for the Deref trait to use. Associated types are a slightly different way of declaring a generic parameter
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// The Drop trait is included in the prelude
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // SHould deallocate memory, but in this example, no need
        println!("Drop");
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn test_mybox() {
    let x = 5;
    let y = MyBox(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // Compiler interprets *y as *(y.deref())

    let n = MyBox("Pierre".to_string());
    hello(&n); // Because Defef is implemented, rust can convert &MyBox<String> in &String. Then stdlib implements Deref that turns &String in &str.
               // Similar to: let n = MyBox::new(String::from("Pierre")); hello(&(*n)[..]);
}

// ---------------------

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn my_drop<T>(x:T) { }

fn test_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    my_drop(d);
    println!("CustomSmartPointer dropped before the end of main.");
}
