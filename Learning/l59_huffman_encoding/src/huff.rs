// huffman.rs (renamed huff.rs since with original name, all text is dimmed, don't know how to clear the cache)
// Huffman class: build encoding dictionary, encoding and decoding
//
// 2025-05-10   PV      First version

//#![allow(unused)]

use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub fn build_encodings_dictionary<T>(arr: &[T]) -> HashMap<T, String>
where
    T: Eq + Hash + Copy + PartialOrd + Display + Debug,
{
    // Count occurrences
    let mut counter: HashMap<T, usize> = HashMap::new();
    for &t in arr {
        (*counter.entry(t).or_default()) += 1;
    }

    // Build priority queue
    let mut pq: BinaryHeap<MyNode<T>> = BinaryHeap::new();
    for item in counter {
        let ln = MyNode::MyLeafNode(LeafNode::new(item.0, item.1));
        pq.push(ln);
    }

    // Aggregate and build tree
    while pq.len() > 1 {
        let n1 = pq.pop().unwrap();
        let n2 = pq.pop().unwrap();
        let n = MyNode::MyInternalNode(InternalNode::new(n1, n2));
        pq.push(n);
    }

    // Build list of encodings
    fn visit<T>(n: &MyNode<T>, ed: &mut HashMap<T, String>, sb: String)
    where
        T: PartialEq + Eq + Hash + Copy + Display + Debug,
    {
        match n {
            MyNode::MyLeafNode(leaf_node) => {
                let sym = leaf_node.symbol;
                ed.insert(sym, sb);
            }
            MyNode::MyInternalNode(internal_node) => {
                visit(&internal_node.left, ed, sb.clone() + "0");
                visit(&internal_node.right, ed, sb.clone() + "1");
            }
        }
    }

    let mut ed = HashMap::<T, String>::new();
    if !pq.is_empty() {
        let root = pq.pop().unwrap();
        visit(&root, &mut ed, String::new());
    }

    ed
}

// Return original list of characters as a long ASCII string containing only '0' and '1'
pub fn get_encoded_bit_string(encodings: &HashMap<char, String>, tc: &[char]) -> String {
    let mut sb = String::new();
    for c in tc {
        sb += encodings[c].as_str();
    }

    sb
}

// -----------------

pub fn get_decoded_bit_string(encodings: &HashMap<char, String>, encoded_bit_string: &str) -> String {
    // At this point, we have an ASCII bitstring, and a hashset of bit_pattern -> char
    // Simply checking if bitstring starts with some bit_ppatern from shortest to longest until we find a match a,d repeating for each char
    // works, but performance would be really bad (O(chars_count*symbols_count*average_symbol_length))
    // A better option is to build a state machine, each bit of bitstring would make progression to next state, or cause an error, or find a char,
    // until bitstring is drained.

    // Quick_and_dirty
    // let mut sorted_keys: Vec<char> = encodings.keys().copied().collect(); // copied() = map(|k| *k)
    // sorted_keys.sort_by_key(|&c| encodings[&c].len());
    // let mut decoded_string = String::new();
    // let mut pos = 0;
    // // Since it's ASCII, we can work at bytes level, no need to care about chars
    // let bs = encoded_bit_string.as_bytes();
    // while pos < bs.len() {
    //     for k in sorted_keys.iter() {
    //         if bs[pos..].starts_with(encodings[k].as_bytes()) {
    //             decoded_string.push(*k);
    //             pos += encodings[k].len();
    //         }
    //     }
    // }

    // Private struct
    #[derive(Debug, Default)]
    struct ZeroOne {
        zero: Option<usize>,
        one: Option<usize>,
        char: Option<char>,
    }

    // Smarter version, build state machine
    let mut states: Vec<ZeroOne> = Vec::new();
    let mut next_state = 1;
    states.push(ZeroOne { ..Default::default() });
    for (ch, e) in encodings {
        // println!("Processing {ch} -> {e}");

        let mut state = 0;
        for &b in e.as_bytes() {
            // println!("  state {state}, bit {}", b as char);
            let ent = states.get_mut(state).unwrap();
            assert!(ent.char.is_none());
            match b {
                b'0' => match ent.zero {
                    Some(next_state) => state = next_state,
                    None => {
                        ent.zero = Some(next_state);
                        state = next_state;
                        next_state += 1;
                        states.push(ZeroOne { ..Default::default() });
                    }
                },
                b'1' => match ent.one {
                    Some(next_state) => state = next_state,
                    None => {
                        ent.one = Some(next_state);
                        state = next_state;
                        next_state += 1;
                        states.push(ZeroOne { ..Default::default() });
                    }
                },
                _ => unreachable!(),
            }
        }
        let ent = states.get_mut(state).unwrap();
        assert!(ent.char.is_none());
        assert!(ent.zero.is_none());
        assert!(ent.one.is_none());
        ent.char = Some(*ch);
    }

    // println!("States count: {}", states.len());
    // for st in &states {
    //     println!("{:?}", st);
    // }
    // println!();

    // Fast decoding using state machine
    let mut decoded_string = String::new();
    let bs = encoded_bit_string.as_bytes();
    let mut state = 0;
    for &b in bs {
        state = if b == b'0' {
            states[state].zero.unwrap()
        } else {
            states[state].one.unwrap()
        };
        if let Some(ch) = states[state].char {
            decoded_string.push(ch);
            state = 0;
        }
    }

    decoded_string
}

// -----------------

#[derive(Debug)]
struct LeafNode<T> {
    pub weight: usize,
    pub symbol: T,
}

impl<T> LeafNode<T>
where
    T: Display,
{
    fn new(symbol: T, weight: usize) -> Self {
        Self { weight, symbol }
    }

    fn node_to_string(&self) -> String {
        format!("LeafNode({}, {}", self.symbol, self.weight)
    }
}

// -----------------

#[derive(Debug)]
struct InternalNode<T> {
    pub weight: usize,
    left: Box<MyNode<T>>,
    right: Box<MyNode<T>>,
}

impl<T> InternalNode<T> {
    pub fn new(left: MyNode<T>, right: MyNode<T>) -> Self {
        Self {
            weight: left.weight() + right.weight(),
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn node_to_string(&self) -> String {
        String::from("InternalNode")
    }
}

impl<T> Display for InternalNode<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "InternalNode Left={}, Right={}, Weight={}",
            self.left.node_to_string(),
            self.right.node_to_string(),
            self.weight
        );
        write!(f, "{}", msg)
    }
}

// -----------------
// C# abstract base class Node, inherited by LeafNode and InternalNode is replaced by an enum in Rust, since traits do
// not support conversion trait -> actual class without lots of extra, while enums easily support it with matching

#[derive(Debug)]
enum MyNode<T> {
    MyLeafNode(LeafNode<T>),
    MyInternalNode(InternalNode<T>),
}

impl<T> MyNode<T>
where
    T: Display,
{
    fn node_to_string(&self) -> String {
        match self {
            MyNode::MyLeafNode(leaf_node) => leaf_node.node_to_string(),
            MyNode::MyInternalNode(internal_node) => internal_node.node_to_string(),
        }
    }
}

impl<T> MyNode<T> {
    fn weight(&self) -> usize {
        match self {
            MyNode::MyLeafNode(leaf_node) => leaf_node.weight,
            MyNode::MyInternalNode(internal_node) => internal_node.weight,
        }
    }
}

// PartialEq, PartialOrd: required for an element of BinaryHeap
impl<T> PartialEq for MyNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl<T> Eq for MyNode<T> {}

impl<T> PartialOrd for MyNode<T>
where
    T: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight().partial_cmp(&self.weight())
    }
}

impl<T> Ord for MyNode<T>
where
    T: PartialEq + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight().cmp(&self.weight())
    }
}
