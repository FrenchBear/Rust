// r13_generics
// Learning rust 2024, The Book §10, 10 Generic Types, Traits, and Lifetimes
//
// 2024-11-20   PV

#![allow(dead_code, unused_variables)]

// Generic function with trait std::cmp::PartialOrd allowing comparison
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generic struct
struct Point<T> {
    x: T,
    y: T,
}

// method on generic struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// method only implemented for Point<f64>
impl Point<f64> {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// generic method on generic struct
struct Pair<X, Y> {
    x: X,
    y:Y,
}

// Need Copy trait to duplicate value (alt: no reference, and take ownership of self and other)
impl<X1: Copy, Y1> Pair<X1, Y1> {
    fn mixup<X2: Copy, Y2>(&self, other: &Pair<X2, Y2>) -> Pair<X1, X2> {
        Pair { x:self.x, y:other.x }
    }
}

// generic enum
enum ThreeContainer<T> {
    Zero,
    One(T),
    Two(T, T),
    Three(T, T, T),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let pinteger = Point { x: 5, y: 10 };
    let pfloat = Point { x: 1.0, y: 4.0 };
    let pointpoint = Point {
        x: Point { x: 1, y: 2 },
        y: Point { x: 3, y: 4 },
    };
    let point1 = pointpoint.x;

    let l = pfloat.length();

    let tc1: ThreeContainer<bool> = ThreeContainer::Zero;
    let tc2 = ThreeContainer::Two(Point { x: 1, y: 2 }, Point { x: 3, y: 4 });
    let tc3 = ThreeContainer::One(tc2);

    let pa1 = Pair {x:12, y:'a'};
    let pa2 = Pair {x:false, y:-2.7};
    let pa3 = pa1.mixup(&pa2);
    
}
