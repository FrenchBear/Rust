// l26_file_hierarchy: Learning Rust, File structures and crates (without cargo)
// rary.rs: Simple library crate
//
// 2025-03-20	PV      First version

pub fn public_function() {
    println!("called rary's `public_function()`");
}
fn private_function() {
    println!("called rary's `private_function()`");
}
pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");
    private_function();
}
