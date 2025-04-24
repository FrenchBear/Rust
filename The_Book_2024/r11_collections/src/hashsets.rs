// r11_collections/hashsets.rs
// Learning rust 2024
//
// 2025-04-24   PV      Added HashSets

use std::collections::HashSet;

pub fn test_hashsets() {
    println!("\ntest_hassets");

    let mut a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = [2, 3, 4, 5].into_iter().collect();

    assert!(a.insert(4)); // Returns true if insert was successful (the set did not contain inserted value)
    assert!(a.contains(&4));

    assert!(b.remove(&2));

    // If a collection's element type implements `Debug`, then the collection implements `Debug`.
    // A HashSet doesn't preserve insertion order, use crate indexmap if needed
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print {1, 2, 3, 4, 5} in arbitrary order
    println!("Union: {:?}", a.union(&b).copied().collect::<Vec<i32>>());
    // This should print {1, 2}
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    // Print {3, 4} in arbitrary order.
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
    // Print {1, 2, 5}
    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());

    // Use extend to add an iterable to a collection
    // drain it an iterator consuming elements
    a.extend(b.drain());
    assert!(b.is_empty());
    b.shrink_to_fit();

    b.insert(11);
    b.insert(12);
    assert!(a.is_disjoint(&b));

    let c: HashSet<i32> = [3, 4].into_iter().collect();
    assert!(c.is_subset(&a));
    assert!(a.is_superset(&c));
    
}
