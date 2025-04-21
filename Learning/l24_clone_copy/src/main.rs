// l24_clone_copy: Learning Rust, Compare Clone and Copy traits
//
// 2025-03-20	PV      First version
// 2025-04-21   PV      Clippy optimizations

#![allow(dead_code)]
#![allow(clippy::clone_on_copy, clippy::non_canonical_clone_impl)]

#[derive(Debug)]
struct Pair(i32, i32);

// Clone is an explicit deep-wise copy
impl Clone for Pair {
    fn clone(&self) -> Self {
        Self(self.0.clone(), -1) // For testing, Clone overrides second member with -1
    }
}

// Copy is a fast, simple a bitwise copy. Must also implement Clone
// Implicit use when assigning to a variable or calling a function
// It's a marker trait, there is no associated method
impl Copy for Pair {}

fn main() {
    let p = Pair(3, 4);
    let p0 = p; // Implicit copy constructor, bitwise fast copy
    let p1 = p.clone(); // Explicit clone, deep copy using explicit code

    println!("p: {:?}", p);
    println!("copy: {:?}", p0); // 2nd member is 4, bitwise copy
    println!("clone: {:?}", p1); // 2nd member is -1, produced by clone()
}

/*
Clone and Copy are traits that deal with duplicating values, but they have distinct purposes and behaviors. Here's a breakdown of their differences:

Copy:

    Implicit Duplication:
        The Copy trait enables implicit duplication of values. When a Copy type is assigned to a new variable or passed as an argument to a function, a bitwise copy is performed.
        This means the original value remains valid and usable.
    Bitwise Copy:
        Copy guarantees that the duplication is a simple, fast, and inexpensive bitwise copy of the memory.
    Restrictions:
        Types that implement Copy must also implement Clone.
        Types containing any form of resource management (like heap-allocated data) generally cannot implement Copy.
        Types that implement the Drop trait, can not implement the Copy trait.
    Marker Trait:
        Copy is a "marker trait," meaning it doesn't define any methods itself. It's simply a signal to the compiler that the type can be copied in this way.
    Example:
        Primitive types like i32, u8, and bool implement Copy.

Clone:

    Explicit Duplication:
        The Clone trait enables explicit duplication of values. You must call the clone() method to create a copy.
        This provides more control over the duplication process.
    Arbitrary Duplication:
        Clone allows for more complex duplication logic. It can perform deep copies, allocate new memory, or execute any necessary operations.
    Flexibility:
        Clone can be implemented for any type, including those that manage resources.
    Method Definition:
        Clone defines the clone() method, which must be implemented by types that implement the trait.
    Example:
        String, Vec<T>, and other complex types implement Clone.

Key Differences Summarized:

    Implicit vs. Explicit: Copy is implicit, Clone is explicit.
    Cost: Copy is always a cheap, bitwise copy; Clone can have varying costs.
    Restrictions: Copy has strict restrictions; Clone is more flexible.
    Purpose: Copy is for simple, easily duplicated values; Clone is for any type that needs to be duplicated.

In essence, Copy is a specialized form of Clone for types that can be safely and efficiently duplicated by simply copying their bits.

*/
