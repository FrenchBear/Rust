// https://github.com/adam-p/markdown-here/wiki/markdown-cheatsheet

use termimad::*;

fn main() {
    let markdown = r#"
# Hello, **World**!

This is some *emphasized* and some _underlined_ text.

- List item 1
- List item 2

```rust
fn main() {
    println!("Hello from Rust!");
}

"#;

    let skin = MadSkin::default();
    println!("{}", skin.term_text(markdown));
}
