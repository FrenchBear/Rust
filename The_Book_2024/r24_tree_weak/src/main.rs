// r24_tree_weak
// Learning rust 2024, Smart Pointers 5: Using Weak<T>
// Prevent references cycles
//
// 2025-01-13   PV

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Creating a Tree Data Structure: a Node with Child Nodes

// We want a Node to own its children, and we want to share that ownership with variables so we can access
// each Node in the tree directly. To do this, we define the Vec<T> items to be values of type Rc<Node>.
// We also want to modify which nodes are children of another node, so we have a RefCell<T> in children
// around the Vec<Rc<Node>>.
#[derive(Debug)]
struct Node {
    value: i32,
    
    parent: RefCell<Weak<Node>>,    
    // A child does not own its parent; when parent is dropped, child nodes should be dropped as well.
    // But if we drop a child node, parent should still exist. This is a case for weak references!

    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Node in leaf has two owners: leaf and branch
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut()=Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // The lack of infinite output indicates that this code didn’t create a reference cycle.
    // We can also tell this by looking at the values we get from calling Rc::strong_count and Rc::weak_count.
}
