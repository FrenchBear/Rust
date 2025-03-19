// l22_format: Learning Rust, specific formatter
// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
//
// 2025-03-17	PV      First version

#![allow(dead_code)]

use std::fmt;

// Define a structure where the fields are nameable
#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

// implement `Display` for `Point2D`, that's the trait where type is unspecified: {}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x={}, y={}", self.x, self.y)
    }
}

// For {:b} formatting
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={:b}, y={:b}", self.x, self.y)
    }
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

// It's tricky because each write! generates a fmt::Result, but we should only provide one...
// Use ? to return in case of error, or continue if write in buffer was successful
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;
        write!(f, "[")?;
        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", self.red, self.green, self.blue)
    }
}

struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.precision() {
            Some(precision) => write!(f, "({:.precision$} + {:.precision$}i)", self.real, self.imag),
            None => write!(f, "({} + {}i)", self.real, self.imag),
        }
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.real, self.imag)
    }
}

struct Rectangular<'a>(&'a Complex);
struct Polar<'a>(&'a Complex);

impl<'a> fmt::Display for Rectangular<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.precision() {
            Some(precision) => write!(f, "({:.precision$} + {:.precision$}i)", self.0.real, self.0.imag),
            None => write!(f, "({} + {}i)", self.0.real, self.0.imag),
        }
    }
}

impl<'a> fmt::Display for Polar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.precision() {
            Some(precision) => write!(f, "({:.precision$} ∠ {:.precision$}ʳ)", self.0.magnitude(), self.0.phase()),
            None => write!(f, "({} ∠ {}ʳ)", self.0.magnitude(), self.0.phase()),
        }
    }
}

// Creating new format specifier :p and :r doesn't work because f.format_spec() is an unstable feature and not directly accessible in stable Rust.
// You can't directly inspect the format string in stable rust.
// impl fmt::Display for Complex {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match f.format_spec().chars().last() {
//             Some('r') => {
//                 let precision = f.precision().unwrap_or(6);
//                 write!(f, "({:.precision$} + {:.precision$}i)", self.real, self.imag, precision = precision)
//             }
//             Some('p') => {
//                 let precision = f.precision().unwrap_or(6);
//                 write!(f, "{:.precision$} * exp({:.precision$}i)", self.magnitude(), self.phase(), precision = precision)
//             }
//             _ => write!(f, "({} + {}i)", self.real, self.imag), // Default formatting
//         }
//     }
// }

fn main() {
    let point = Point2D { x: -8, y: 14 };
    println!("Debug: {:?}", point);
    println!("Debug pretty: {:#?}", point);
    println!("Display: {}", point);
    println!("Display binary: {:b}", point);

    let v = List(vec![1, 2, 3]);
    println!("\nList: {}", v);

    println!("\nColors:");
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("{}", color);
    }

    println!("\nComplex numbers:");
    let z = Complex { real: 3.0, imag: 4.0 };
    println!("z: {:.3}", z);
    println!("z: {}", z);
    println!("z: {:.3}", Polar(&z));
    println!("z: {}", Polar(&z));
}
