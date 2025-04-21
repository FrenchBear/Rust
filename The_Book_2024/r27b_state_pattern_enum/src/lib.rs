// Implementation of a state pattern
// Learning Rust, OO Design Patterns
//
// 2025-02-12   PV      Personal implementation, using an enum, seems simpler and easier to read to me
// 2025-04-21   PV      Clippy suggestions

enum PostState {
    Draft,
    PendingReview,
    Approved,
}

pub struct Post {
    state: PostState,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: PostState::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        match self.state {
            PostState::Approved => panic!("Can't update an approved post!"),
            _ => self.content.push_str(text),
        }
    }

    pub fn content(&self) -> &str {
        match self.state {
            PostState::Approved => &self.content,
            _ => "",
        }
    }

    pub fn request_review(&mut self) {
        match self.state {
            PostState::Draft => self.state = PostState::PendingReview,
            PostState::PendingReview => {}
            PostState::Approved => panic!("Can't review an approved post!"),
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            PostState::Draft => panic!("Can't approve a draft!"),
            PostState::PendingReview => self.state = PostState::Approved,
            PostState::Approved => {}
        }
    }
}

// Suggestion from Clippy
impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}
