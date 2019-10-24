fn main() {
    let mut post = Post::new();

    post.add_text("Wubba lubba.. ");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("bud bud!");
    assert_eq!("", post.content());

    // request rejection moves it back to draft state
    post.reject();
    post.add_text("dub dub!");
    assert_eq!("", post.content());

    post.approve();
    post.add_text(" Peace among worlds!");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("bud bud!");
    assert_eq!("", post.content());

    // Need two level approval
    post.approve();
    post.add_text("bud bud!");
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Wubba lubba.. dub dub! Peace among worlds!", post.content());
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn can_add_text(&self) -> bool {
        false
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {approval_count: 0})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
       self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add_text(&self) -> bool {
        true
    }
}

struct PendingReview {
    approval_count: i8,
}


impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        match self.approval_count {
            1 => {
                self.approval_count += 1;
                Box::new(Published {})
            },
            0 => {
                self.approval_count += 1;
                self
            },
            _ => self,
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
