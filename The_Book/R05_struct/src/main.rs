// R05_struct
// Learning Rust
//
// 2018-10-21	PV
// 2023-05-16   PV      Restarted Rust learning

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, inner: &Rectangle) -> bool {
        self.width>=inner.width && self.height>=inner.height
    }

    // An associated function doesn't need self parameter (so it's not a method), for instance to implement a factory.
    // Use :: to call an associated function.
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
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
    println!("r area: {}", r.area());

    let r = Rectangle::square(2);
    println!("carre area: {}", r.area());

    let mut black = Color(0, 0, 0);
    black.print();
    black.invert();
    black.print();

    let rect1 = Rectangle {
        width: 30, height: 50,
    };
    let rect2 = Rectangle {
        width: 10, height: 40,
    };
    let rect3 = Rectangle {
        width: 60, height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}
