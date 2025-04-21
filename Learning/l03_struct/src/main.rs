// l03_struct
// Learning Rust again, Structures
//
// 2023-05-16   PV

#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct User {
    active: bool,
    signin_count: u32,
    nom: String,
    email: String,
}

// Tuple struct, Color and Point are different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
// Not clear at this stage
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define a method, that is, a function within the context of Rectangle, we start an impl (implementation) block for Rectangle.
// Methods can take ownership of self (actually it's rare), borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.
impl Rectangle {
    fn area(&self) -> u32 {
        // self is a shortcut for self: &Self within an impl block.
        self.height * self.width
    }
}

fn main() {
    let u1 = create_user(
        String::from("Pierre"),
        String::from("pierre.violent@gmail.com"),
    );
    println!("{:?}", u1);

    // u2 is initialized from u1, but since User does not implement Copy trait, String nom is moved from u1, so u1 is no longer valid after this
    let u2 = User {
        email: String::from("pierre.violent@outlook.com"),
        ..u1
    };
    println!("{:#?}", u2); // Pretty print using :#?
    //println!("{:?}", u1);     // borrow of partially moved value: `u1` partial move occurs because `u1.nom` has type `String`, which does not implement the `Copy` trait

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let r1 = Rectangle {
        width: dbg!(2 + 2),
        height: 3,
    };
    dbg!(&r1); // Need & otherwise dbg! macro takes ownership of r1.  dbg! needs #[derive(Debug)] on struct
    println!("r1 surface: {}", r1.area());
}

fn create_user(nom: String, email: String) -> User {
    let signin_count = 1;
    User {
        active: true,
        signin_count,
        nom,
        email,
    }
}
