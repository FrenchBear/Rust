// R05_struct
// Learning Rust
// 2018-10-21	PV

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Use a tuple
struct Color(u8, u8, u8);

impl Color {
    fn invert(&mut self) {
        self.0 = !self.0;
        self.1 = !self.1;
        self.2 = !self.2;
    }

    fn print(&self) {
        println!("({}, {}, {})", self.0, self.1, self.2);
    }
}

fn main() {
    let r = Rectangle {
        width: 8,
        height: 7,
    };
    println!("Area: {}", r.area());

    let mut black = Color(0, 0, 0);
    black.print();
    black.invert();
    black.print();
}
