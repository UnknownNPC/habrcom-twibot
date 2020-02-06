use super::abstraction::CommentPublisher;
use std::error::Error;

pub struct TwitterCommentsPublisher;

impl CommentPublisher for TwitterCommentsPublisher {
    fn publish_comment(message: &str) -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
}

