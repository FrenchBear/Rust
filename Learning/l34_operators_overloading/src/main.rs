// l34_operators_overloading
// Learning Rust, Rust by Exaample, §16.3
// Also example of Drop trait implementation (just one method, drop)
//
// 2025-04-03   PV      First version

use std::ops;

struct Foo;
struct Bar;
#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

impl Drop for FooBar {
    fn drop(&mut self) {
        println!("A FooBar was dropped");
    }
}

impl Drop for BarFoo {
    fn drop(&mut self) {
        println!("A BarFoo was dropped");
    }
}
