extern crate tokio;

use egg_mode::tweet::DraftTweet;
use tokio::runtime::current_thread::block_on_all;

use super::abstraction::CommentPublisher;

pub struct TwitterCommentsPublisher {
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
}

impl TwitterCommentsPublisher {
    pub fn new(consumer_key: String, consumer_secret: String, access_key: String, access_secret: String) -> TwitterCommentsPublisher {
        TwitterCommentsPublisher {
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
        }
    }
}

impl CommentPublisher for TwitterCommentsPublisher {
    fn publish_comment(self, message: &String) -> Result<String, String> {
        println!("Publishing message: [{}]", message);

        let con_token = egg_mode::KeyPair::new(self.consumer_key, self.consumer_secret);
        let access_token = egg_mode::KeyPair::new(self.access_key, self.access_secret);
        let token = egg_mode::Token::Access {
            consumer: con_token,
            access: access_token,
        };

        let draft = DraftTweet::new(message);

        let response = block_on_all(draft.send(&token));

        match response {
            Ok(tweet) => Ok(tweet.id.to_string()),
            Err(e) => Err(format!("Failed with error: [{}]", e)),
        }
    }
}
