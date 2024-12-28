// r18_closures
// Learning rust 2024
//
// 2024-12-10   PV

#![allow(dead_code, unused_variables)]

use std::thread;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum ShirtColor {
    Red,
    Blue,
    White,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        if let Some(userprefcolor) = user_preference {
            if self.shirts.contains(&userprefcolor) {
                return userprefcolor;
            }
        }
        self.most_stocked()
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut counter: HashMap<ShirtColor, i32> = HashMap::new();
        for color in &self.shirts {
            let count = counter.entry(*color).or_insert(0);
            *count += 1;
        }

        let mut colors_counts:Vec<(&ShirtColor, &i32)> = counter.iter().collect();
        colors_counts.sort_by_key(|kv| -kv.1);
        let color_most = colors_counts.first().unwrap().0;      // Will panic if stock is empty, ok for now
        *color_most
    }
}

fn giveaway_tshirt() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let user_pref3 = Some(ShirtColor::White);
    let giveaway3 = store.giveaway(user_pref3);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref3, giveaway3
    );

}

fn borrow_immutably() {
    println!("\nborrow_immutably");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn borrow_mutably() {
    println!("\nborrow_mutably");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mut = || list.push(7);
    borrows_mut();
    println!("After calling closure: {list:?}");
}

fn take_ownership() {
    println!("\ntake_ownership");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Without moving value in the closure, thread executing yake_ownership() could terminate before spawned thread execures,
    // making list reference invalid (closure may outlive current function)
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

fn main() {
    giveaway_tshirt();

    borrow_immutably();
    borrow_mutably();
    take_ownership();
}
