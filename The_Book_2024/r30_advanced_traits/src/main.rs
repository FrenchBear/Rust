// r30_advanced_traits
// Learning rust 2024, Advanced features
//
// 2025-02-18   PV

#![allow(unused, static_mut_refs)]

use std::ops::Add;

mod disambiguation;
mod supertraits;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Implementing the Add trait to overload + operator for Point instances
// Add trait has an associated type named Output that determines the type returned from the add method
// Here is the definition of Add trait:
//
// trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
// Rhs defaults to Self if we don't specify a concrete type for Rhs when implementing Add, making it easier to implement

impl Add for Point {
    type Output = Point;        

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// Example of implementation of Add specifying Rhs
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    disambiguation::disambiguation_methods();
    disambiguation::disambiguation_associated_functions();

    supertraits::supertraits_example();
}
