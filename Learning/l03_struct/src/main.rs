// l03_struct
// Learning structures
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
struct AlwaysEqual;

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
    println!("{:?}", u2);
    //println!("{:?}", u1);       // borrow of partially moved value: `u1` partial move occurs because `u1.nom` has type `String`, which does not implement the `Copy` trait

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
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
