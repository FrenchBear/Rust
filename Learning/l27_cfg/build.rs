// simple example of build script

fn main() {
    println!("cargo::rustc-check-cfg=cfg(pi4)");
    println!("cargo::rustc-cfg=pi4");
    println!("cargo build")
}
