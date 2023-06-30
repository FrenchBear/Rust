// l18_rc
// Learning Rust again
//
// Rc<T> is only for use in single-threaded scenarios.
// Rc<T> handles immutable references
//
// 2023-06-29   PV

#![allow(unused, non_snake_case)]

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    simple_cloning();
    show_count();
    test_dog();
}

fn simple_cloning() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Cloning an Rc<T> Increases the Reference Count
    let b = Cons(3, Rc::clone(&a)); // Rc::clone doesn't make a deep copy, but increment reference count
    let c = Cons(4, Rc::clone(&a)); // Rc::clone(&a) is identical to a.clone()
}

fn show_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// -----

struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Drop {}", self.name);
    }
}

fn test_dog() {
    println!("\nDog");
    let owner1 = Rc::new(Dog {
        name: String::from("Medor"),
    });
    let owner2 = Rc::clone(&owner1);
    println!("Owners: {}", Rc::strong_count(&owner1));
    drop(owner1);
    drop(owner2);
    println!("After drops");
}
