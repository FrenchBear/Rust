// R10_generics
// Learning Rust
// 2018-11-10	PV

#![allow(dead_code)]
#![allow(unused_variables)]

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    // This implementation moves value out of self and other...
    fn mixup<V, W>(self, other: Point<V,W>) -> Point<T, W> {
        Point {x: self.x, y:other.y}
    }
}


fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.414, y: 1.732 };

    let p3 = p1.mixup(p2);
    println!("p3: ({}, {})", p3.x, p3.y);

    let a1 = [1,4,3,12,3,5];
    let a2 = [3.1416, 2.7182, 1.4142, 1.7321];
    let a3 = ["once", "upon", "a", "time"];
    println!("largest(a1): {}", largest(&a1));
    println!("largest(a2): {}", largest(&a2));
    println!("largest(a3): {}", largest(&a3));
}


// Wait until I know how to add a trait constraint to a generic function
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
