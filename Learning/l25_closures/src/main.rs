// l25_closures: Learning Rust, functions returning closures
//
// 2025-03-20	PV      First version
// 2025-04-21   PV      Clippy optimizations

#![allow(dead_code, unused_variables)]
#![allow(clippy::non_canonical_clone_impl)]

// Return a closure, since x and k support Copy, move is enough to capture k by value in the closure
fn offset<F>(f: F, k: u32) -> impl Fn(u32) -> bool
where
    F: Fn(u32) -> bool,
{
    move |x| f(k + x)
}

#[derive(Debug)]
struct Metre(f64);

// Explicit copy, in-depth
impl Clone for Metre {
    fn clone(&self) -> Self {
        println!("Clone metre({})", self.0);
        //Self(self.0)
        *self
    }
}

// Implicit bitwise copy
impl Copy for Metre {}

fn mo(k: f64) -> impl Fn(f64) -> Metre {
    move |x: f64| Metre(x + k)
}

// Here we need metre to implement Clone or Copy to the move closure can capture k by value
fn mx(k: Metre) -> impl Fn(Metre) -> Metre {
    // move |m: Metre| k.clone()        // This version explicitly calls clone(), so Copy trait is not required
    move |m: Metre| k // Implicit copy, Copy trait is required
}

fn main() {
    let k = Metre(2.0);
    let f = mx(k);
    let g = f(Metre(1.0));
    println!("g: {:?}", g)
}
