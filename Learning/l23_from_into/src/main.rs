// l23_from_into: Learning Rust, conversions
// https://doc.rust-lang.org/rust-by-example
//
// 2025-03-19	PV      First version

#![allow(dead_code)]

use std::convert::From;

// Define a structure where the fields are nameable
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl From<Point2D> for Point3D {
    fn from(value: Point2D) -> Self {
        Point3D {
            x: value.x,
            y: value.y,
            z: 0.0,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl From<Point2D> for Complex {
    fn from(value: Point2D) -> Self {
        Complex {
            real: value.x,
            imag: value.y,
        }
    }
}

// Conversion from value consumes the value
impl From<Complex> for Point2D {
    fn from(value: Complex) -> Self {
        Point2D {
            x: value.real,
            y: value.imag,
        }
    }
}

// Need a conversion from reference to preserve original during conversion
impl From<&Complex> for Point2D {
    fn from(value: &Complex) -> Self {
        Point2D {
            x: value.real,
            y: value.imag,
        }
    }
}

fn main() {
    let p2 = Point2D { x: -2.5, y: 3.7 };
    println!("p2: {:?}", p2);
    let p3 = Point3D::from(p2);
    println!("p3: {:?}", p3);

    let z = Complex { real: 3.0, imag: 4.0 };
    println!("z: {:?}", z);
    let pz2 = Point2D::from(&z);
    println!("pz2: {:?}", pz2);
    let z2 = Complex::from(pz2);
    println!("z2: {:?}", z2);
    assert!(z == z2);
}
