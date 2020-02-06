extern crate config;

mod message_provider;
mod message_publisher;

use message_provider::HabrCommentsProvider;
use message_provider::CommentProvider;

use message_publisher::CommentPublisher;
use message_publisher::TwitterCommentsPublisher;

fn main() {

    let mut settings = config::Config::new();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let topics_url: String = settings.get("habr_topics_page").unwrap();

    println!("Searching for comment using next page: [{}]", topics_url);

    let get_comment = HabrCommentsProvider::get_comment(topics_url);
    println!("Random comment: {}", get_comment)


}
