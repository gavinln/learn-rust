use ood_pattern::Post;

use core::cell::RefCell;

fn main() {
    // object-oriented design (OOD) pattern
    // Functionality
    // 1. Blog post starts as an empty draft
    // 2. When the draft is done a review is requested
    // 3. When the post is approved, it gets poublished
    // 4. Only published posts return content to print
    // 5. Unapproved posts can't be accidentally published

    // implemented using Box<dyn State> - run time checks
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("Draft: {}", post.content());
    // Draft - request_review -> PendingReview, approve -> self
    post.request_review();
    println!("PendingReview: {}", post.content());
    // PendingReview - request_review -> self, approve -> Published
    post.approve();
    println!("Published: {}", post.content());
    // Published - request_review -> self, approve -> self

    // implemented using multiple types - compile time checks
    let mut post = Post2::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post2 {
    content: String,
}

pub struct Draft2 {
    content: String,
}

pub struct PendingReview2 {
    content: String,
}

impl Post2 {
    pub fn new() -> Draft2 {
        Draft2 {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Draft2 {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReview2 {
        PendingReview2 {
            content: self.content,
        }
    }
}

impl PendingReview2 {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}
