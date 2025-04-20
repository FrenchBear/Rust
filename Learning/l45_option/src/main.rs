// l45_option: Learning Rust
// Use ? in a function returning Option<T>, chaining ?
//
// 2025-04-18	PV      First version

#![allow(dead_code)]

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Gets the area code of the phone number of the person's job, if it exists.
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        // It would take a lot more code - try writing it yourself and see which is easier.
        self.job?.phone_number?.area_code
    }
}

// -----------------------------------------------------
// Option::map (map combinator): apply the function to the x value in Some(x) or preserves None
// - Some(x).map(fn) -> Some(fn(x))
// - None.map(fn) -> None
// It's a chainable way to simplify match statements

fn test_option_map() {
    let n = Some(42);
    let o = n.map(|x| x + 1);
    assert_eq!(o, Some(43));

    // Note that applyying a function returning Option<U> will return an Option<Option<U>> --> see .and_then
    let s = Some(String::from("Hello"));
    let t = s.map(|st| st.find('k')); // t is an Option<Option<usize>>
    assert_eq!(t, Some(None));
}

// -----------------------------------------------------
// and_then
// Using map() on a function that returns an Option<T> results in the nested Option<Option<T>>.
// Chaining multiple calls together can then become confusing.
// That's where another combinator called and_then(), known in some languages as flatmap, comes in.

fn test_option_and_then() {
    let s = Some(String::from("Hello"));
    let t = s
        .map(|s| format!("«{}»", s)) // map fn(T) -> U
        .and_then(|st| st.find('k')) // and_then fn(T) -> Some(U)
        .and_then(|x| Some(x + 1));
    // t is an Option<size>
    assert_eq!(t, None);
}

// -----------------------------------------------------
// Unpacking options and defaults

#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

// or() is chainable and eagerly evaluates its argument, and because of that, the variable passed to or is moved.
fn test_option_or() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;
    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}\n", first_available_fruit);

    // In the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
    // But the variable named `apple` has been moved regardless, and cannot be used anymore.
}

// or_else() is a version of or(), chainable, that evaluates lazily
fn test_option_or_else() {
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit.or_else(get_kiwi_as_fallback).or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}\n", first_available_fruit);
}

// get_or_insert() evaluates eagerly, modifies empty value in place
// To make sure that an Option contains a value, we can use get_or_insert to modify it in place with a fallback value.
// Note that get_or_insert eagerly evaluates its parameter, so variable apple is moved
fn test_option_get_or_insert() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}\n", my_fruit);
    // Here apple is not available anymore
}

// get_or_insert_with() evaluates lazily, modifies empty value in place
// Instead of explicitly providing a value to fall back on, we can pass a closure to get_or_insert_with
fn test_option_get_or_insert_with() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    // Providing lemon as fallback
    // first_available_fruit is: Lemon
    // my_fruit is: Some(Lemon)

    // If the Option has a value, it is left unchanged, and the closure is not invoked
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}\n", my_apple);
    // The output is a follows. Note that the closure `get_lemon_as_fallback` is not invoked
    // should_be_apple is: Apple
    // my_apple is unchanged: Some(Apple)
}

// -----------------------------------------------------

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));
    println!();

    test_option_map();
    test_option_and_then();

    test_option_or();
    test_option_or_else();
    test_option_get_or_insert();
    test_option_get_or_insert_with();

    let oi: Option<i32> = Some(42);
    let oj = oi.filter(|x| x & 1 == 0); // Filter using predicate: Value None->None, Some(x): if predicate is true -> Some(x) else None
    assert_eq!(oi, Some(42));

    oi.inspect(|x| println!("Value {} is even.\n", x)); // Call function(x) if value is Some(x)

    // replace: return old value, and remplace inner value by new value (can't replace by None)
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);

    // take: takes the value out of the option, leaving a [None] in its place.
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);

    // fn take_if(&mut self, predicate: P) -> Option<i32>
    // Takes the value out of the option, but only if the predicate evaluates to true on a mutable reference to the value.
    // In other words, replaces self with None if the predicate returns true. This method operates similar to [Option::take] but conditional.
    let mut x = Some(42);
    let prev = x.take_if(|v| {
        if *v == 42 {
            *v += 1;
            false
        } else {
            false
        }
    });
    assert_eq!(x, Some(43));
    assert_eq!(prev, None);

    let prev = x.take_if(|v| *v == 43);
    assert_eq!(x, None);
    assert_eq!(prev, Some(43));

    // fn xor(self, optb: Option<i32>) -> Option<i32>
    // Returns [Some] if exactly one of self, optb is [Some], otherwise returns [None].
    let x = Some(2);
    let y: Option<u32> = None;
    assert_eq!(x.xor(y), Some(2));

    let x: Option<u32> = None;
    let y = Some(2);
    assert_eq!(x.xor(y), Some(2));

    let x = Some(2);
    let y = Some(2);
    assert_eq!(x.xor(y), None);

    let x: Option<u32> = None;
    let y: Option<u32> = None;
    assert_eq!(x.xor(y), None);

    let x: Option<u32> = Some(2);
    let y: Option<u32> = Some(4);
    assert_eq!(x.xor(y), None);

    // fn and(self, optb: Option<U>) -> Option<U>
    // Returns [None] if the option is [None], otherwise returns optb.
    // Arguments passed to and are eagerly evaluated; if you are passing the result of a function call, 
    // it is recommended to use and_then, which is lazily evaluated.
    let x = Some(2);
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);

    let x: Option<u32> = None;
    let y = Some("foo");
    assert_eq!(x.and(y), None);

    let x = Some(2);
    let y = Some("foo");
    assert_eq!(x.and(y), Some("foo"));
    println!("y: {:?}", y);     // Works, because &str implements Copy (but not Clone)

    let x: Option<u32> = None;
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);

    let x: Option<u32> = None;
    let y = Some("foo");
    assert_eq!(x.and(y), None);

    let app = Some(Fruit::Apple);
    let ora = Some(Fruit::Orange);
    let zz = app.and(ora);
    //println!("ora: {:?}", ora);    // Error, ora has been moved since Fluit doesn't implement Copy (or define enum Fruit with #[derive(Debug, Clone, Copy)])



}
