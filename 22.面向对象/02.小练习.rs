fn main() {
    // run_enum()
    run_multiple_type()
}
fn run_enum() {
    enum State {
        NoApproved,
        Approved,
    }
    struct Post {
        content: String,
        state: State,
    }
    impl Post {
        fn new() -> Self {
            Post {
                content: String::new(),
                state: State::NoApproved,
            }
        }
        fn add_text(&mut self, str: &str) {
            self.content.push_str(str);
        }
        fn request_review(&mut self) {}
        fn approve(&mut self) {
            self.state = State::Approved;
        }
        fn content(&self) -> &str {
            match &self.state {
                State::Approved => &self.content,
                _ => "",
            }
        }
    }
    let mut post = Post::new();
    post.add_text("111");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("111", post.content());
}
// 状态编码为不同的类型
fn run_multiple_type() {
    struct Draftpost {
        content: String,
    }
    impl Draftpost {
        fn add_text(&mut self, str: &str) {
            self.content.push_str(str);
        }
        fn request_review(&mut self) -> PendingPreviewPost {
            PendingPreviewPost {
                content: self.content.clone(),
            }
        }
    }
    struct PendingPreviewPost {
        content: String,
    }
    impl PendingPreviewPost {
        fn arrpove(&self) -> Post {
            Post {
                content: self.content.clone(),
            }
        }
    }
    #[derive(Debug)]
    struct Post {
        content: String,
    }
    impl Post {
        fn new() -> Draftpost {
            Draftpost {
                content: String::new(),
            }
        }
    }
    let mut post = Post::new();
    post.add_text("111");
    let post = post.request_review();
    println!("{:?}", post.arrpove())
}
