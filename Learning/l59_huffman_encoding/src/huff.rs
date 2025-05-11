// huffman.cs
// Huffman class: build encoding dictionary
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

    let root = pq.pop().unwrap();
    let mut ed = HashMap::<T, String>::new();
    visit(&root, &mut ed, String::new());

    ed
}

// Return original list of characters as a long ASCII string containing only '0' and '1'
pub fn get_encoded_bit_string(tc: &[char], encodings: &HashMap<char, String>) -> String {
    let mut sb = String::new();
    for c in tc {
        sb += encodings[c].as_str();
    }

    sb
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
