// r09_restaurant
// Learning rust 2024, The Book §7 example, Play with modules, binary crate of restaurant
//
// 2024-11-10   PV

#![allow(dead_code, unused_variables)]

//use r10_restaurant::{self, eat_breakfast_at_restaurant};
use r10_restaurant::*;

fn main() {
    println!("Hello from main");
    
    eat_breakfast_at_restaurant();
    eat_at_restaurant();
}

use r10_restaurant::front_of_house::hosting;

fn aw1() {
    hosting::add_to_waitlist();
}

// use is only for the scope in which use appears.
mod customer {
    fn aw2() {
        // in mod customer, use of hosting causes an error.
        //hosting::add_to_waitlist();

        // But a relative access to the parent is Ok
        super::hosting::add_to_waitlist();
    }
}

// Note: full access to fuction is Ok, but not idiomatic
use r10_restaurant::front_of_house::hosting::add_to_waitlist;

fn aw3() {
    add_to_waitlist();
}

// On the other hand, when bringing in structs, enums, and other items with use, 
// it’s idiomatic to specify the full path.
use std::collections::HashMap;

fn test_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Exception: we can't bring the same name twice into scope, for instance, use std::fmt::Result; 
// and use std::io::Result; is not permitted
// But it's possible to rename one, for instance, use std::io::Result as IoResult;

