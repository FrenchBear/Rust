// l06_modules
// Learning Rust again, Simple example of code in two files
// See l07_restaurant for more complex modules
//
// 2023-06-10   PV

mod garden;
mod lake;

use crate::garden::vegetables::Asparagus;

fn main() {
    println!("Hello, world!");
    garden::flower();
    lake::pond();

    let plant = Asparagus{};
    println!("Growing {:?}", plant);
}
