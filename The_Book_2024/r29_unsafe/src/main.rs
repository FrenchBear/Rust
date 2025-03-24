// r29_unsafe
// Learning rust 2024, Advanced features
//
// 2025-02-17   PV

#![allow(unused, static_mut_refs)]

use std::slice;

// Mutable static variable, reading or writing to it is unsafe, any code doing is must be in an unsafe block
// Multiple thread access could cause a data race consition (better use thread-safe smart pointers in this case)
static mut COUNTER: u32 = 0;

fn main() {
    // We can create raw pointers in safe code, we can't dereference them outside an unsafe block
    // Note that we can have at the same time a mutable and an immutable reference pointing to the same location,
    // this is not allowed with standard Rust references
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Raw pointer to an arbitrary address in memory (creating a pointer does no harm)
    let address = 0x012345usize;
    let r = address as *const i32;

    // Dereferencing raw pointers
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Call unsafe function
    unsafe {
        dangerous();
    }

    // Calling (a safe) function containing an unsafe block
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Using extern to call external code
    unsafe extern "C" {
        unsafe fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Reading a mutable global static variable is unsafe
    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

// Unsafe function (body of an unsafe function is an unsafe block)
unsafe fn dangerous() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("Value of *r1 before update through *r2: {}", *r1);
        *r2 = 7;
        println!("Value of *r1 after update through *r2: {}", *r1);
    }
}

// Rust does not know we're borrowing different parts of the slice, and that's Ok because parts are non-overlapping
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // Takes a raw pointer and a length to build a slice
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Function modifying a mutable global static variable
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// FUnction that can be called from other languages
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Unsafe traits
unsafe trait Foo {
    // methods go here
}
unsafe impl Foo for i32 {
    // method implementations go here
}
