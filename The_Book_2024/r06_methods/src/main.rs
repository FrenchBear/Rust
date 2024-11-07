// r06_methods
// Learning rust 2024
//
// 2024-11-07   PV

#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Define an area method on the rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This can't be confused with field with. Without parentheses, r.width is the field, with parentheses, r.field() is a method call
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width<=self.width && other.height<=self.height
    }

    // Associated function (doesn't have &self parameter contrary to methods)
    // To call it, we must use the Rectangle:: prefix
    // Self is an alias for the type appearing after impl, Rectangle here
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    // Simple tuple (directly supports Debug trait)
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!( "The area of the rectangle {:?} is {} square pixels.", rect1, rect1.area());
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
}
