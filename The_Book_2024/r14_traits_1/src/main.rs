// r14_traits
// Learning rust 2024, The Book §10, 10 Generic Types, Traits, and Lifetimes
//
// 2024-11-21   PV
// 2025-02-13   PV      Added example showing how to declare a variable of type Trait

#![allow(dead_code, unused_variables)]

use std::f64::consts::PI;
use std::fmt::{self, Display};

// Define a trait
pub trait Surface2D {
    fn surface(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Trait with default implementation
    fn name(&self) -> &str {
        "Surface2D"
    }

    // A trait with default impl can call other traits with or without default impl
    fn to_string(&self) -> String {
        format!(
            "{}: surface={}, perimeter={}",
            self.name(),
            self.surface(),
            self.perimeter()
        )
    }
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Surface2D for Circle {
    fn surface(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    // No implementation of fn name(&self) for Circle
}

pub struct Rectangle2points {
    pub top_left_corner: Point,
    pub bottom_right_corner: Point,
}

impl Surface2D for Rectangle2points {
    fn surface(&self) -> f64 {
        (self.bottom_right_corner.x - self.top_left_corner.x)
            * (self.bottom_right_corner.y - self.top_left_corner.y).abs()
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.bottom_right_corner.x - self.top_left_corner.x).abs()
            + 2.0 * (self.bottom_right_corner.y - self.top_left_corner.y).abs()
    }

    // Override default impl
    fn name(&self) -> &str {
        "Rectangle"
    }
}

// Another member function, independently from Surface2D Trait
impl Rectangle2points {
    fn center(&self) -> Point {
        Point {
            x: (self.top_left_corner.x + self.bottom_right_corner.x) / 2.0,
            y: (self.top_left_corner.y + self.bottom_right_corner.y) / 2.0,
        }
    }
}

// Use directly trait in function interface
fn print_surface_1(surf: &dyn Surface2D) {
    println!("{} surface_1 = {}", surf.name(), surf.surface());
}

// Alt, use &impl Trait instead of type
fn print_surface_2(surf: &impl Surface2D) {
    println!("{} surface_2 = {}", surf.name(), surf.surface());
}

fn max_surface_2(surf1: &impl Surface2D, surf2: &impl Surface2D) {
    // surf1 and surf2 don't need to have the same type
    let ms = if surf1.surface() >= surf2.surface() {
        surf1.name()
    } else {
        surf2.name()
    };
    print!("Max surface_2: {}", ms)
}

// Trait applied to generic type (trait bound)
fn print_surface_3<T: Surface2D>(name: &str, surf: &T) {
    println!("{} surface_3 = {}", name, surf.surface());
}

// Using where clause on generic type
fn print_surface_4<T>(name: &str, surf: &T)
where
    T: Surface2D,
{
    println!("{} surface_3 = {}", name, surf.surface());
}

fn max_surface_3<T: Surface2D>(surf1: &T, surf2: &T) {
    // surf1 and surf2 must have the same type
    let ms = if surf1.surface() >= surf2.surface() {
        surf1.name()
    } else {
        surf2.name()
    };
    print!("Max surface_3: {}", ms)
}

// Implement library trait Display on local type
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

trait Half {
    fn get_half(&self) -> Self;
}

impl Half for Point {
    fn get_half(&self) -> Point {
        Self {
            x: self.x / 2.0,
            y: self.y / 2.0,
        }
    }
}

// Use + to specify multiple trait bounds
fn print_half_3<T: Half + Display>(p: &T) {
    let h = p.get_half();
    println!("{}", h);
}

// use where clause
fn print_half_4<T>(p: &T)
where
    T: Half + Display,
{
    let h = p.get_half();
    println!("{}", h);
}

// Returning a type that implements Traits
fn returns_surface() -> impl Surface2D {
    Rectangle2points {
        top_left_corner: Point { x: -2.0, y: 3.0 },
        bottom_right_corner: Point { x: 5.0, y: -3.3 },
    }
}

// ----------------------------------------------------------
// Implement a method conditional to trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

// constructor/factory method
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Method only for T that implement Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// ----------------------------------------------------------

pub trait MonTrait {
    fn say_hello(&self);
}

pub struct Machin {
    pub x: i32,
}

impl MonTrait for Machin {
    fn say_hello(&self) {
        println!("Hello!");
    }
}

// ----------------------------------------------------------

fn main() {
    let c = Circle {
        center: Point { x: 2.0, y: 3.0 },
        radius: 2.5,
    };
    let r = Rectangle2points {
        top_left_corner: Point { x: 1.0, y: 4.0 },
        bottom_right_corner: Point { x: 3.0, y: 2.0 },
    };

    print_surface_1(&c);
    print_surface_1(&r);

    print_surface_3("Circle c", &c);
    print_surface_3("Rectangle r", &r);
    println!("{}", r.to_string());

    let p = Point { x: 3.8, y: -1.2 };
    print_half_3(&p);
    print_half_4(&p);

    let s2 = returns_surface();
    print_surface_1(&s2);

    // Use trait Trait: dyn Trait
    let ma = Machin { x: 3 };
    let mb: Box<dyn MonTrait> = Box::new(Machin { x: 3 }); // Use Box<dyn Trait> to get a reference (and because the size for any type implementing Box isn't known at compile time of this line)
    let x1: &dyn MonTrait = &ma;
    let x2: &dyn MonTrait = &(*mb); // Can't use directly mb, need to dereference the box and then take the address
}
