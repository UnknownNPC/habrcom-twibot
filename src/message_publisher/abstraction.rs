pub trait CommentPublisher {
    fn publish_comment(self, message: &String) -> Result<String, String>;
}
