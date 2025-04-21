// Advanced traits, disambiguation
//
// 2025-02-18   PV

// Calling Methods (=having a &self parameter) with the same name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn disambiguation_methods() {
    let person = Human;

    person.fly(); // Call method implemented on Human directly (prints *waving arms furiously*)
    // Specifying trait name before method to select specific version of fly()
    Pilot::fly(&person);
    Wizard::fly(&person);
}

// calling associated funtions of the same name
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn main() {
    println!("A baby dog is called a {}", Dog::baby_name()); // Calls the function defined on Dog directly (Spot)
    //println!("A baby dog is called a {}", Animal::baby_name());   // cannot call associated function on trait without specifying the corresponding `impl` type

    // To disambiguate and tell Rust that we want to use the implementation of Animal for Dog as opposed to the implementation of Animal
    // for some other type, we need to use fully qualified syntax.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // "Cast" a type, not a variable

    // fully qualified syntax is defined as follows:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}
