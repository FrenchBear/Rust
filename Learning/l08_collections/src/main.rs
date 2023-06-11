// l08_collections
// 2023-06-11   PV

#![allow(unused_variables)]

fn main() {
    // Arrays (init: https://www.joshmcguigan.com/blog/array-initialization-rust/)
    let mut a1: [i32; 10] = [0; 10]; // Initialization from [T; N] where T: Copy
    let a1b: [i32; 10] = Default::default(); // Initialization from [T; N] where T: Default (and N <= 32)
    let a2 = [1, 2, 3];
    let a3 = ["Once", "upon", "a", "time"];
    a1[0] = 5;
    a1[1] = 6;
    a1[2] = 7;

    // Vectors
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let v3 = vec!["Once", "upon", "a", "time"];
    v1.push(5);
    v1.push(6);
    v1.push(7);
    let third = v1[2];      // Ok since integers support copy
    v1[2] = -3;
    println!("Third: {third}");
    let third: &i32 = &v1[2];
    println!("Third: {third}");
    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("Third: {third}"),
        None => println!("No third."),
    }
    let third: Option<i32> = v1.get(2).copied();    // copied: Maps an Option<&T> to an Option<T> by copying the contents of the option
    match third {
        Some(third) => println!("Third: {third}"),
        None => println!("No third."),
    }

    // let vs:Vec<String> = vec![String::from("Hello"), String::from("world"),];
    // let s = vs[1];  // move occurs because value has type `String`, which does not implement the `Copy` trait
    // println!("s={s}");

    
}
