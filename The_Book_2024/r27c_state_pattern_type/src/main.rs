// r27c_state_pattern_type
// Learning rust 2024, Object oriented design pattern
// Version c, alternate implementation from the Book
//
// 2025-02-13   PV

#![allow(unused)]

use r27c_state_pattern_type::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
