// r26_traits
// Learning rust 2024, Object features of Rust
//
// 2025-02-11   PV

#![allow(unused)]

pub trait Draw {
    fn draw(&self);
}

// Implementation using a trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementation using a generic type implementing a trait
pub struct  Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T> where T:Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementing the trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // relevant code
        println!("Draw a Button");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // relevant code
        println!("Draw a SelectBox");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {width:75, height:10, options:vec![String::from("Yes"), String::from("No"), String::from("Maybe")]}),
            Box::new(Button { width:50, height:10, label:String::from("OK")}),
        ]
    };

    screen.run();
}
