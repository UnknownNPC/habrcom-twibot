pub trait CommentProvider {
    fn get_comment(url: String) -> String;
}
