// l31_ref_to_temp
// Learning Rust, not clear how I can return a ref to temp in random_animal, and even less clear,
// Why it doesn't work once I implement Drop trait...
//
// 2025-03-28	PV      First version

struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// impl Drop for Sheep {
//     fn drop(&mut self) {
//         println!("Sheep's dead.")
//     }
// }

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// impl Drop for Cow {
//     fn drop(&mut self) {
//         println!("Cow's dead.")
//     }
// }

// Returns some struct that implements Animal, but we don't know which one at compile time.
// Apparently I can create a temp object, and return a reference to it in some cases, athough
// it doesn't work if I implement Drop trait...
fn random_animal<'a>(random_number: f64) -> &'a dyn Animal {
    if random_number < 0.5 {
        &Sheep {}
    } else {
        &Cow {}
    }
}
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
