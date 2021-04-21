pub mod blog {
    pub struct Post {
        content: String
    }

    impl Post {
        pub fn new() -> Draft {
            Draft {
                content: String::new()
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }


    pub struct Draft {
        content: String
    }


    impl Draft {
        pub fn add_text(&mut self, text: &str) -> () {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> InitialReview {
            InitialReview {
                content: self.content
            }
        }
    }

    pub struct InitialReview {
        content: String
    }

    impl InitialReview {
        pub fn request_review(self) -> FinalReview {
            FinalReview{
                content: self.content
            }
        }

        pub fn reject(self) -> Draft {
            Draft {
                content: self.content
            }
        }

    }

    pub struct FinalReview {
        content: String
    }

    impl FinalReview {
        pub fn approve(self) -> Post {
            Post {
                content: self.content
            }
        }
        pub fn reject(self) -> Draft {
            Draft {
                content: self.content
            }
        }
    }


}
