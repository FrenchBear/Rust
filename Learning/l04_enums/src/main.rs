// l04_enums
// Learning Rust again, PLay with enum and Option
//
// 2023-05-17   PV
// 2025-04-21   PV      Clippy optimizations

#![allow(unused_variables, dead_code, unused_mut)]

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    V7 { addr: String, color: String },
    V8(String, String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match x {
    //     None => None,
    //     Some(v) => Some(v + 1),
    // }
    // Better:
    x.map(|n| n + 1)
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let v7 = IpAddr::V7 {
        addr: String::from("Unknown"),
        color: String::from("Transparent"),
    };
    let v8 = IpAddr::V8(String::from("A"), String::from("B"));

    let n1 = Some(1);
    let n2 = n1.clamp(Some(0), Some(10));
    let n3 = n1.and(n2); // Return n2 only if n1 and n2 both have a value, else None

    let mut some_value = Some(1);
    // match some_value.as_mut() {
    //     Some(v) => *v = 42,
    //     None => {}
    // }
    // Better:
    if let Some(ref mut v) = some_value {
        *v = 42;
    }
    println!("v={:?}", some_value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(x) = six {
        println!("six={x}");
    }
}
