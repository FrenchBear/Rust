// r27b_state_pattern_enum
// Learning rust 2024, Object oriented design pattern
// Version b, personal implementation, using an enum
//
// 2025-02-13   PV

#![allow(unused)]

use r27b_state_pattern_enum::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // post.add_text("\r\nText updated after post approval");
    // println!("{}", post.content());
}
