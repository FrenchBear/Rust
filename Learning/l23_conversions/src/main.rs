// l23_conversions: Learning Rust, conversions, Rust by Example, §6
// https://doc.rust-lang.org/rust-by-example
//
// 2025-03-19	PV      First version

#![allow(dead_code, unused_variables)]

use std::{
    convert::From,
    fmt::{self, Display},
    str::FromStr,
};

use regex::Regex;

// Define a structure where the fields are nameable
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Automatically provides ToString conversion
impl Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2D({}, {})", self.x, self.y)
    }
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

// TryFrom is for conversions that can fail

// Define a custom error type.
#[derive(Debug)]
pub struct ConversionError {
    message: String,
}

impl ConversionError {
    pub fn new(message: String) -> ConversionError {
        ConversionError { message }
    }
}

// Implement the Display trait to make the error printable.
impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConversionError: {}", self.message)
    }
}

// Implement the Error trait to make it a proper error type.
impl std::error::Error for ConversionError {}

impl TryFrom<&Point3D> for Point2D {
    //type Error = ();    // Just return error, don't bother for details
    type Error = ConversionError;

    fn try_from(value: &Point3D) -> Result<Self, Self::Error> {
        if value.z == 0.0 {
            Ok(Point2D { x: value.x, y: value.y })
        } else {
            Err(ConversionError {
                message: "Z coordinate is not nul, can't convert Point3D to Point2D".into(),
            })
        }
    }
}

#[derive(Debug)]
struct Point2Di32 {
    x: i32,
    y: i32,
}

impl Into<Point2D> for Point2Di32 {
    fn into(self) -> Point2D {
        // Note that the signature use from, while From:: does not
        Point2D {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl Into<Point2D> for &Point2Di32 {
    // Preserve original, here self is actually &Point2Di32
    fn into(self) -> Point2D {
        // Note that the signature use from, while From:: does not
        Point2D {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

// Conversion string -> Point2Di32, supports for str.parse::<Point2Di32>
impl FromStr for Point2Di32 {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\( *(\d+) *, *(\d+) *\)").unwrap();
        let ca = match re.captures(s) {
            Some(ca) => ca,
            None => {
                return Err(ConversionError {
                    message: "Invalid string format for Point2Di32".into(),
                });
            }
        };
        let x = match ca[1].parse::<i32>() {
            Ok(x) => x,
            Err(e) => {
                return Err(ConversionError {
                    message: format!("Invalid x coordinate: {}", e),
                });
            }
        };
        let y = match ca[2].parse::<i32>() {
            Ok(y) => y,
            Err(e) => {
                return Err(ConversionError {
                    message: format!("Invalid y coordinate: {}", e),
                });
            }
        };
        Ok(Point2Di32 { x: x, y: y })
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
    let p2a = Point2D { x: -2.5, y: 3.7 };
    println!("p2a: {:?}", p2a);
    let p3a = Point3D::from(p2a);
    println!("p3a: {:?}", p3a);

    let p2 = Point2D { x: 4.0, y: -1.0 };
    let s = p2.to_string(); // Provided automatically by Display trait

    let p2b = Point2D { x: 0.1, y: 4.2 };
    let p3b: Point3D = p2b.into(); // into() calls automatically from() if needed

    let z = Complex { real: 3.0, imag: 4.0 };
    println!("z: {:?}", z);
    let pz2 = Point2D::from(&z);
    println!("pz2: {:?}", pz2);
    let z2 = Complex::from(pz2);
    println!("z2: {:?}", z2);
    assert!(z == z2);

    let k1 = Point2Di32 { x: 4, y: 0 };
    let k2 = Into::<Point2D>::into(&k1); // Complex syntax if we don't want to use explicitly typed target variable
    let k3: Point2D = (&k1).into(); // Simpler version
    //let k4 = Point2D::from(k1);                    // into() calls from() if needed, but the reciprocal is not true

    let p3 = Point3D { x: 0.4, y: 4.5, z: 1.3 };
    let res = Point2D::try_from(&p3);
    match res {
        Ok(c) => println!("Conversion Ok, result = {:?}", c),
        Err(e) => println!("Conversion error: {}", e),
    }

    let a1 = "(5, 7)".parse::<Point2Di32>();
    println!("a1 = {:?}", a1);
    let a2 = "5; 7)".parse::<Point2Di32>();
    println!("a1 = {:?}", a2);
    let a3 = "(1234567890123456, 7)".parse::<Point2Di32>();
    println!("a1 = {:?}", a3);
}
