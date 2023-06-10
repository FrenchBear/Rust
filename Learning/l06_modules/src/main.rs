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
