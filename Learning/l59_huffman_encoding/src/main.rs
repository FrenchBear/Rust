// l59_huffman_encoding
// Play with Huffman encoding
//
// 2025-05-10   PV      First version from C#, basically spent 6 hours to convert almost trivial code...

// Difficulties:
//
// Using a trait common to LeafNode and InternalNode: overcomplicated, can't match a trait to instances of actual types,
// you need to add your own discriminant function, and then do unsafe obscure conversions to convert trait -> actual
// struct (see l60_downcast_trait_without_any).
//
// Using a tree of references, where InternalNode left and right are references doesn't work, you need to create a temp
// InternalNode, but then it doesn't survive to be able to add it to the tree. Solution could be a pseudo-static vector
// with pre-allocated max size to store new InternalNodes and being sure that references stored in the vector won't
// change, but that requires "magic" type coercion... Finally I've decided to store InternalNode directly inside the
// tree, so they have an owner.
//
// Implementing a chained hierarchy with children pointing to Parent is impossible with references (simple &T and
// Option<&T>) and lifetime control, besides the "lifetime cancer", there is no way to create a new parent object and
// update children contained is not possible with simple code. A solution would combine Rc<T>, RefCell<T>, Weak<T>,
// Option<> and other extra-heavy encapsulation of these types, and since they're references, we're back to previous
// issue of a tree of references.
//
// Finally, I've removed the parent reference, and the visitor pattern is building binary encoded representation from
// top to bottom, instead of bottom to top used in C#. Implementation of browsing from LeafNode to root is
// overcomplicated in Rust (and even if my code did compile, it didn't work...). The good news is that the top-down
// approach used here is simpler than C# code, and eliminates parent reference and all lifetimes.
//
// Conclusion: Don't even thing of translating code from a managed memory language to Rust, lifetimes and ownership
// block many simple constructions allowed by managed memory, and it requires new algorithms a a deep rewrite from
// scratch using different structures and different code.  Short version, porting code from C# to Rust is virtually
// impossible besides trivial code using only static data.

// ToDo: Write conversion result in a file
// ToDo: Decode converted file
// ToDo: Add test cases (beware that there is no guaranted/unique order for symbols of same encoded length)

//#![allow(unused)]

use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;

fn main() {
    process_string("A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED");
}

fn process_string(s: &str) {
    let v: Vec<char> = s.chars().collect();
    let encodings = build_encodings_dictionary(&v);

    let mut original_length = 0;
    let mut encoded_length = 0;
    let mut max_encoded_symbol_length = 0;
    for c in &v {
        original_length += c.len_utf8() * 8;
        let e = encodings.get(c).unwrap();
        let le = e.len();
        encoded_length += le;
        if le > max_encoded_symbol_length {
            max_encoded_symbol_length = le;
        }
    }

    println!("{:?}", encodings);
    println!("{} characters to encode", v.len());
    println!("Original length: {} bits (UTF-8)", original_length);
    println!(
        "Encoded length: {} bits, {:.3} bits per character, {:.1}% of original length",
        encoded_length,
        encoded_length as f64 / v.len() as f64,
        100.0 * encoded_length as f64 / original_length as f64
    );
    println!("Max encoded bits per symbol: {}", max_encoded_symbol_length);


}

fn build_encodings_dictionary<T>(arr: &[T]) -> HashMap<T, String>
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

// -----------------

trait Node<T> {
    fn weight(&self) -> usize;
}

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

impl<T> Node<T> for LeafNode<T>
where
    T: Display,
{
    fn weight(&self) -> usize {
        self.weight
    }
}

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
            self.weight()
        );
        write!(f, "{}", msg)
    }
}

impl<T> Node<T> for InternalNode<T> {
    fn weight(&self) -> usize {
        self.weight
    }
}

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
