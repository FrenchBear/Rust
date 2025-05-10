// l61_priority_queue
// How to implement a simple PriorityQueue (min-heap, returns elements from lowest priority to highest)
// BinaryHeap struct from the std::collections implements a max-priority queue
//
// 2025-05-10   PV

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Item<T>(T, usize);

impl<T> PartialOrd for Item<T>
where
    T: PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // REVERSE ORDER!
        other.1.partial_cmp(&self.1)
    }
}

impl<T> Ord for Item<T>
where
    T: PartialEq + Eq + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // REVERSE ORDER!
        other.1.cmp(&self.1)
    }
}

fn main() {
    let mut p: BinaryHeap<Item<String>> = BinaryHeap::new();

    p.push(Item("une".into(), 3));
    p.push(Item("fois".into(), 4));
    p.push(Item("Il".into(), 1));
    p.push(Item("était".into(), 2));

    while !p.is_empty() {
        let item = p.pop().unwrap();
        println!("{}", item.0);
    }
}
