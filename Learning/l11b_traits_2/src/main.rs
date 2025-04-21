// l11b_traits_2
// Learning Rust again, converting Struct -> Trait -> Struct
//
// 2025-03-28   PV

#![allow(unused)]

use std::any::Any;
use std::f64::consts::PI;
use std::fmt::Debug;

trait Surface {
    fn area(&self) -> f64;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Surface for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Square {
    side: f64,
}

#[derive(Debug)]
struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

fn main() {
    let c = Circle { radius: 1.54 };
    println!("Circle surface 2: {}", c.area());

    // Convert a Circle in a Surface, v1
    let ds: &dyn Surface = &c;
    println!("Circle surface 2: {}", ds.area());

    // Convert a Circle in a Surface, v2
    let bds: Box<dyn Surface> = Box::new(Circle { radius: 3.72 });
    println!("Circle surface 3: {}", (*bds).area());

    // Casting back from &dyn Surface to &Circle
    if let Some(circle_ref) = ds.as_any().downcast_ref::<Circle>() {
        println!("Circle radius: {}", circle_ref.radius);
        println!("Circle area: {}", circle_ref.area());
    } else {
        println!("Not a circle");
    }

    let s = Square { side: 4.28 };
    let t = Triangle {
        side1: 3.0,
        side2: 4.0,
        side3: 5.0,
    };
    print_info(&c);
    print_info(&s);
    print_info(&t);
}

fn print_info<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;

    if let Some(c) = value_any.downcast_ref::<Circle>() {
        println!("It's a circle, radius={}, area={}", c.radius, c.area());
    } else if let Some(s) = value_any.downcast_ref::<Square>() {
        println!("It's a square, side={}", s.side);
    } else {
        println!("Neither a circle not a square: {:?}", value);
    }
}
