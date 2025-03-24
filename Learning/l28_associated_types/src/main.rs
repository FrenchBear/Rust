// l28_associated_types: Learning Rust, Generics, Accociated types
//
// 2025-03-24	PV      First version

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, n1: &i32, n2: &i32) -> bool {
        self.0 == *n1 && self.1 == *n2
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
    container.last() - container.first()
}


trait Kontains {
    type A;
    type B;

    fn kontains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn kfirst(&self) -> i32;
    fn klast(&self) -> i32;
}


impl Kontains for Container {
    // Specify what types `A` and `B` are. If the `input` type is `Container(i32, i32)`, the `output` types are determined as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn kontains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn kfirst(&self) -> i32 { self.0 }
    // Grab the last number.
    fn klast(&self) -> i32 { self.1 }
}

fn kdifference<C: Kontains>(container: &C) -> i32 {
    container.klast() - container.kfirst()
}


fn main() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));


    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.kontains(&number_1, &number_2));
    println!("First number: {}", container.kfirst());
    println!("Last number: {}", container.klast());
    
    println!("The difference is: {}", kdifference(&container));

}
