// lib.rs for traits
// Learning Rust again
//
// 2023-06-19   PV

pub trait Summary {
    fn summarize(&self) -> String;

    fn comment(&self) -> String {
        // Default implementation
        "No comment".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Book {
    pub title: String,
    pub author: String,
}
impl Summary for Book {
    fn summarize(&self) -> String {
        format!("Book {} from {}", self.title, self.author)
    }

    fn comment(&self) -> String {
        "Book-specific comment".to_string()
    }
}
