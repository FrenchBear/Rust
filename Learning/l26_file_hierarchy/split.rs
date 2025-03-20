// l26_file_hierarchy: Learning Rust, File structures and crates (without cargo)
// split.rs: Entry point for binary crate
//
// 2025-03-20	PV      First version

// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

extern crate rary;      // Actually not needed

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();

    rary::public_function();
    rary::indirect_access();
}
