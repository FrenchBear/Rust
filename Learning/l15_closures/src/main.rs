// l15_closures
// Learning Rust again
//
// 2023-06-27   PV

#![allow(unused, non_snake_case)]

use num;
use std::{collections::HashMap, ops::Add};

fn main() {
    capture_immutable_reference();
    capture_mutable_reference();
    closure_moved_to_a_new_thread();
    functions_also_implement_FnOnce();

    test_rectangle();
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum ShirtColor {
    Red,
    Blue,
    Green,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_green = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Green => num_green += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    // More general implementation, supports more than 2 colors, if there is no stock for user_preference returns item with most stock,
    // if there is no stock, returns None
    fn giveaway_v2(&self, user_preference: Option<ShirtColor>) -> Option<ShirtColor> {
        // Build a counter indexed by color
        let mut cnt: HashMap<ShirtColor, usize> = HashMap::new();
        for color in &self.shirts {
            let c = cnt.entry(*color).or_insert(0);
            *c += 1;
        }

        // If there's a user preference and stock for this color, returns it
        if let Some(c) = user_preference {
            if *cnt.get(&c).unwrap_or(&0) > 0 {
                return Some(c);
            }
        }

        // Compute item with max stock
        let mut cmax = ShirtColor::Blue; // Dummy initialization to make compiler happy
        let mut imax = 0;
        for (c, i) in cnt {
            if i > imax {
                cmax = c;
                imax = i;
            }
        }
        if imax > 0 {
            Some(cmax) // If there(s any stock, returns item with most stock)
        } else {
            None // Nothing left
        }
    }
}

fn tshirts_giveway() {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn there_is_stock() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway_v2(user_pref1);
        assert_eq!(giveaway1, Some(ShirtColor::Red));
    }

    #[test]
    fn there_is_no_stock() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Green);
        let giveaway1 = store.giveaway_v2(user_pref1);
        assert_eq!(giveaway1, Some(ShirtColor::Blue));
    }

    #[test]
    fn no_user_preference() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = None;
        let giveaway1 = store.giveaway_v2(user_pref1);
        assert_eq!(giveaway1, Some(ShirtColor::Blue));
    }

    #[test]
    fn no_stock() {
        let store = Inventory { shirts: vec![] };

        let user_pref1 = None;
        let giveaway1 = store.giveaway_v2(user_pref1);
        assert_eq!(giveaway1, None);
    }
}

fn fn_vs_closures_vs_generic() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;
    fn add_one_v5<T>(x: T) -> T
    where
        T: Add<Output = T> + num::Num,
    {
        x + T::one()
    }

    let i = add_one_v1(add_one_v2(add_one_v3(add_one_v4(1u32))));
    let j = add_one_v5(1i32); // Generic allow multiple types, can't be done with closures
    let k = add_one_v5(1u32);

    let add_one_v5_i128 = add_one_v5::<i128>;
    let l = add_one_v5_i128(7324975923532959i128);
}

fn capture_immutable_reference() {
    println!("\ncapture_immutable_reference");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list); // It's a simple Fn
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn capture_mutable_reference() {
    println!("\ncapture_mutable_reference");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7); // It's a FnMut
                                               // Can't put a println! here since there because there's a mutable borrow in progress, an immutable borrow is not allowed
                                               //println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

use std::thread;

fn closure_moved_to_a_new_thread() {
    println!("\nclosure_moved_to_a_new_thread");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // move converts any variables captured by reference or mutable reference to variables captured by value.
    // list must be moved to the thread in case calling thread terminates first
    // Without move: error[E0373]: closure may outlive the current function, but it borrows `list`, which is owned by the current function
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    //println!("After thread: {:?}", list);     // error[E0382]: borrow of moved value: `list`
}

fn test_closure() {
    let s = String::from("Hello");
    let w = String::from("World");
    let l = || s + "!"; // Closure takes ownership of s. Type FnOnce, can only be called pnce.
    let t = l(); // Not possible with FnOnce
                 //let u = l();    // Use ov moved value

    //println!("{s}");    // Err borrow ov moved value s
    println!("{w}");
}

// -------------------

enum MyOption<T> {
    MySome(T),
    MyNone,
}

impl<T> MyOption<T> {
    // Genenric type F is only related to the function unwrap_or_else, not to the enum or impl
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T, // Note that Fn and FnMut also implement FnOnce trait
    {
        match self {
            MyOption::MySome(x) => x,
            MyOption::MyNone => f(),
        }
    }
}

// -------------------

fn trois() -> i32 {
    3
}

fn functions_also_implement_FnOnce() {
    println!("\nfunctions_also_implement_FnOnce");
    let o: Option<i32> = None;
    let i = o.unwrap_or_else(trois);
    println!("{i}")
}

// -------------------

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test_rectangle() {
    println!("\ntest_rectangle");
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // pub fn sort_by_key<K, F>(&mut self, f: F)
    // where
    //   F: FnMut(&T) -> K,
    //   K: Ord,
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // Weird: sort_by_key requires that closure has FnMut trait, and I don't think that sw has it, but it's Ok...
    let swh = |r: &Rectangle| r.height;
    list.sort_by_key(swh);
    println!("{:#?}", list);

    // swm is a FnMut
    let mut cnt = 0;
    let swm = |r: &Rectangle| {
        cnt += 1;
        r.width
    };
    list.sort_by_key(swm);
    println!("{:#?} cnt={cnt}", list);
}
