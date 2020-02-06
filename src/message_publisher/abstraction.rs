use std::error::Error;

pub trait CommentPublisher {
    fn publish_comment(message: &str) -> Result<String, Box<dyn Error>>;
}
