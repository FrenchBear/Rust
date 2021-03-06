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
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Specific implementation for <f64,f64>
impl Point<f64, f64> {
    fn distance_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Specific implementation for <i32,i32>
impl Point<i32, i32> {
    fn distance_origin(&self) -> i32 {
        max(self.x.abs(), self.y.abs())
    }
}

fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.414, y: 1.732 };

    println!("p1.distance_origin(): {}", p1.distance_origin());
    println!("p2.distance_origin(): {}", p2.distance_origin());

    let p3 = p1.mixup(p2);
    println!("p3: ({}, {})", p3.x, p3.y);

    let a1 = [1, 4, 3, 12, 3, 5];
    let a2 = [3.1416, 2.7182, 1.4142, 1.7321];
    let a3 = ["once", "upon", "a", "time"];
    println!("largest(a1): {}", largest1(&a1));
    println!("largest(a2): {}", largest1(&a2));
    println!("largest(a3): {}", largest1(&a3));
}

// Version using references
fn largest1<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Version using Copy trait (value types)
fn largest2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Version 3 using Clone trait
fn largest3<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}
