// r11_collections/vectors.rs
// Learning rust 2024, The Book §8, common collections
//
// 2024-11-10   PV
// 2025-04-21   PV      Clippy suggestions

#![allow(clippy::or_then_unwrap)]

pub fn test_vectors() {
    println!("\ntest_vectors");

    let mut v: Vec<i32> = Vec::new(); // Vec<T> is generic, so we must add type info to the variable for type inference to work
    let mut u = [1, 2, 3]; // Or use vec! macro that create with values, type is inferred from the values

    v.push(5);
    v.push(6);

    let third = u[2];
    let third_ref = &u[2];
    println!("Third element of u: {third} or {third_ref}");

    // tryGetValue
    let third_opt = u.get(2);
    match third_opt {
        Some(v) => "u has a third element, value={v}", // Some(&v) makes directy v a i32 instead of a &i32
        None => "u doesn't have a third element",
    };

    // Convert a Option<&i32> into a i32 with a default value of -1 without using a match construct, but it seems overly complex
    let zz = *u.get(2).or(Some(&-1)).unwrap();
    let uu = *u.get(2).unwrap_or(&-1);
    let tt = u.get(2).copied().unwrap_or(-1); // Solution, copied is the magic that converts Option<&T> into Option<T> where T:Copy

    u[2] = 5;
    let u2 = u[2];
    println!("Now, u[2] is {u2}");

    //println!("Previously, u[2] was {third_ref}");     // Can't use a borrowed ref
    println!("Previously, u[2] was {zz}"); // But a dereferenced value is Ok

    // -------------------------
    // Iterating over values

    // Immutable references
    for i in &v {
        print!("{i} ");
    }
    println!();

    // Dereferenced immutable references for simple scalars
    for &i in &v {
        print!("{i} ");
    }
    println!();

    // Moving the ownership of values out of v, after this, v can't be used
    for i in v {
        print!("{i} ");
    }
    println!();

    // Can't use v now
    // let v0 = v[0];
    // println!("v0={v0}");

    // Mutable reference when iterating
    // Note that it is forbidden to add a remove values to a vector inside an iteration loop over it (mutable or not)
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // Use dereference operator
    }

    // Get and remove the last element (added with push) from a vector (use last() to get the element without removing it)
    // last() -> Option<&T>, pop() -> Option(T)
    let last1 = v.last().copied().unwrap();
    let len1 = v.len();
    let last2 = v.pop().unwrap(); // unwrap because pop() returns Option<T>
    let len2 = v.len();
    assert_eq!(last1, last2);
    assert_eq!(len1, len2 + 1);

    // Vectors containing different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
