// R08_collections
// Learning Rust
// 2018-10-29	PV

#![allow(unused_mut)]
#![allow(unused_variables)]

fn main() {
    vectors();
}

fn vectors() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = Vec::<i32>::new();
    let mut v3 = vec![1, 2, 3]; // Declare and initialize: no type annotation needed

    v1.push(1);
    v1.push(2);
    v1.push(3);

    // Indexed access
    let third: &i32 = &v1[2];
    let trois: i32 = v1[2];
    //v1.push(4);       // Not accepted because there is an immutable borrow 2 lines above

    // Iterate over mutable references (can't do it on v1 since there is an immutable bowwow)
    for i in &mut v3 {
        println!("{}", i);
        *i += 100;
    }

    // get accessor returning Option<&T>.
    let v_index = 5;
    match v1.get(v_index) {
        Some(_) => { println!("Reachable element at index: {}", v_index); }
        None => { println!("Unreachable element at index: {}", v_index); }
    }

    // use enums to store more than one type in a vector
    enum Mixed {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Mixed::Int(3),
        Mixed::Text(String::from("blue")),
        Mixed::Float(10.12),
    ];

}
