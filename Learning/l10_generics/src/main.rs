// l10_generics
// Learning Rust again
//
// 2023-06-19   PV
// 2025-04-21   PV      Clippy optimizations

#![allow(unused)]

// Note that a call using a &Vec<i32> can match a &[i32] parameter...
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            // Also correct: if *item > *largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result1 = largest_i32(&number_list);
    let result2 = largest(&number_list);
    println!("The largest number is {result1} or {result2}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result1 = largest_char(&char_list);
    let result2 = largest(&char_list);
    println!("The largest char is {result1} or {result2}");

    // Use ::<type> to select a specific generic function
    let largest_f64 = largest::<f64>;

    let fruit_list = vec![
        Fruit::new("Poire"),
        Fruit::new("Pomme"),
        Fruit::new("Ananas"),
    ];
    let result = largest(&fruit_list);
    println!("The largest fruit is {result:?}");

    let pi = Point { x: 3, y: 4 };
    let pf: Point<f64> = Point { x: 0.866, y: 0.5 };
    let pu: Point<u8> = Point { x: 6, y: 12 };
    println!("pf: x={} y={} l={:.4}", pf.x, pf.y, pf.module());

    let mp1 = MixedPoint { x: 1, y: 'w' };
    let mp2 = MixedPoint {
        x: "Hello",
        y: 12.331,
    };
    let mp3 = MixedPoint::mixup(mp2, mp1);
    println!("mp3: x={} y={}", mp3.x, mp3.y);
}

// Generic version
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Use default implementation for PartialEq and PartialOrd...
#[derive(Debug, PartialEq, PartialOrd)]
struct Fruit {
    pub name: String,
}

impl Fruit {
    fn new(n: &str) -> Fruit {
        Fruit {
            name: n.to_string(),
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn module(&self) -> f64 {
        f64::hypot(self.x, self.y)
    }
}

impl Point<f32> {
    fn module(&self) -> f32 {
        f32::hypot(self.x, self.y)
    }
}

struct MixedPoint<X, Y> {
    x: X,
    y: Y,
}

impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixedPoint<X2, Y2>) -> MixedPoint<X1, Y2> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}
