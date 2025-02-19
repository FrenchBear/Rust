// Advanced traits, supertraits
//
// 2025-02-18   PV

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Without this, couldn't implement OutlinePrint for Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

pub fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
    //OutlinePrint::outline_print(&p);      // Same thing

    let r: &dyn OutlinePrint = &p;
    r.outline_print();

    (&p as &dyn OutlinePrint).outline_print();

    let q: Box<dyn OutlinePrint>;
    q = Box::new(p);        // Consumes p
    (*q).outline_print();
}
