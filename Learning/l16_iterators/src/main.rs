// l16_iterators
// Learning Rust again
//
// 2023-06-27   PV
// 2025-04-21   PV      Clippy optimizations

#![allow(unused, non_snake_case)]

use std::{collections::btree_map::Iter, ops::Add};

fn main() {
    simple_iter();
    next_demo();
    iterator_sum();
    adapter();
    test_Σ();
}

fn simple_iter() {
    let v1 = [1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        // the loop took ownership of v1_iter and made it mutable behind the scenes
        println!("Got: {}", val);
    }
}

fn next_demo() {
    let v1 = [1, 2, 3];
    let mut v1_iter = v1.iter(); // Must be mutable because its internal state changes after each call to next. This code consumes the iterator.
    assert_eq!(v1_iter.next(), Some(&1)); // next returns an immutable reference
    assert_eq!(v1_iter.next(), Some(&2)); // If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter
    assert_eq!(v1_iter.next(), Some(&3)); // if we want to iterate over mutable references, we can call iter_mut instead of iter
    assert_eq!(v1_iter.next(), None);
}

pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// consuming adapters
fn iterator_sum() {
    let v1 = [1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on
}

// Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
// Instead, they produce different iterators by changing some aspect of the original iterator.
fn adapter() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.into_iter().map(|x| x + 1).collect(); // into_iter takes ownership of values (x is i32, not &i32 as we get with iter), v1 can't be used after that
    assert_eq!(v2, vec![2, 3, 4]);

    let euler_gamma = 0.5772156649015328; // More or less...
    let r = 1..=10000;
    let l: f64 = r.map(|x| 1.0 / x as f64).sum::<f64>() - euler_gamma; // FINALLY found the way to select a specific generic function
    println!("l={} ln(1000)={}", l, f64::ln(10000.0));
}

fn multiples(number: i32, upto: i32) -> Vec<i32> {
    (1..=upto).filter(|n| n % number == 0).collect()
}

fn Σ<T>(items: impl Iterator<Item = T>) -> T
where
    T: Add<Output = T> + num::Num,
{
    let mut s = T::zero();
    for item in items {
        s = s + item;
    }

    s
}

fn test_Σ() {
    println!("\nTest Σ");

    let v1 = vec![1, 2, 3, 4, 5];
    let s1 = Σ(v1.into_iter());
    println!("s1={s1}");

    let v2 = vec![1.1, 2.2, 3.3];
    let s2 = Σ(v2.into_iter());
    println!("s2={s2}");
}
