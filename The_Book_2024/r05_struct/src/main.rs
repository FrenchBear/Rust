// structs, tuples structs and tuples
// Learning rust 2024
//
// 2024-11-07   PV

#![allow(dead_code, unused_variables)]

// A struct, with named fields
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs, without named fields, but different types
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);


// Unit-like structs without fields!
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user1: {user1:?}");
    println!("user1, alt: {user1:#?}");
    dbg!(&user1);

    user1.email.push_str(",anotheremail@example.com");

    // Construct a new user with few changes from another user
    // BEWARE! Since we use =, we MOVE the value of user1.username, so user1.username is not available anymore!
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let em = user1.email;       // Still Ok
    //let un = user1.username;          // Not Ok anymore

    // Initialize Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {black:?}");
    println!("origin: {origin:?}");

    // Initializa a struct without any fields
    let subject = AlwaysEqual;

    // Simple tuple (directly supports Debug trait)
    let rect1 = (30, 50);
    println!( "The area of the rectangle {:?} is {} square pixels.", rect1, area(rect1));
    dbg!(&rect1);
}

// Field init shortcut (same name as variable)
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // instead of username: username
        email,    // instead of email: email
        sign_in_count: 1,
    }
}

// Simple tuple parameter
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
