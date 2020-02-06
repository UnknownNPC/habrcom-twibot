extern crate config;

use std::env;

use message_provider::CommentProvider;
use message_provider::HabrCommentsProvider;
use message_publisher::CommentPublisher;
use message_publisher::TwitterCommentsPublisher;

mod message_provider;
mod message_publisher;

const HABR_TOPICS_URL_CONFIG_KEY: &'static str = "habr_topics_page";
const HABR_TOPICS_URL_ENV_KEY: &'static str = "HABR_TOPICS_PAGE";
const TWITTER_API_CONSUME_KEY_CONFIG_KEY: &'static str = "twitter_api_consume_key";
const TWITTER_API_CONSUME_KEY_ENV_KEY: &'static str = "TWITTER_API_CONSUME_KEY";
const TWITTER_API_CONSUME_SECRET_CONFIG_KEY: &'static str = "twitter_api_consume_secret";
const TWITTER_API_CONSUME_SECRET_ENV_KEY: &'static str = "TWITTER_API_CONSUME_SECRET";

const TWITTER_API_ACCESS_KEY_CONFIG_KEY: &'static str = "twitter_api_access_key";
const TWITTER_API_ACCESS_KEY_ENV_KEY: &'static str = "TWITTER_API_ACCESS_KEY";
const TWITTER_API_ACCESS_SECRET_CONFIG_KEY: &'static str = "twitter_api_access_secret";
const TWITTER_API_ACCESS_SECRET_ENV_KEY: &'static str = "TWITTER_API_ACCESS_SECRET";

fn main() {
    fn search_conf_param(file_conf: &config::Config, conf_key: &str, env_key: &str) -> String {
        file_conf
            .get(&conf_key)
            .or(env::var(env_key))
            .expect(&format!(
                "Please set `{}` config or `{}` env var",
                conf_key, env_key
            ))
    }

    let mut settings = config::Config::new();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let topics_url: String = search_conf_param(
        &settings,
        &HABR_TOPICS_URL_CONFIG_KEY,
        &HABR_TOPICS_URL_ENV_KEY,
    );

    let twitter_consume_key: String = search_conf_param(
        &settings,
        &TWITTER_API_CONSUME_KEY_CONFIG_KEY,
        &TWITTER_API_CONSUME_KEY_ENV_KEY,
    );

    let twitter_consume_secret: String = search_conf_param(
        &settings,
        &TWITTER_API_CONSUME_SECRET_CONFIG_KEY,
        &TWITTER_API_CONSUME_SECRET_ENV_KEY,
    );
    let twitter_access_key: String = search_conf_param(
        &settings,
        &TWITTER_API_ACCESS_KEY_CONFIG_KEY,
        &TWITTER_API_ACCESS_KEY_ENV_KEY,
    );

    let twitter_access_secret: String = search_conf_param(
        &settings,
        &TWITTER_API_ACCESS_SECRET_CONFIG_KEY,
        &TWITTER_API_ACCESS_SECRET_ENV_KEY,
    );

    println!("Searching for comment using next page: [{}]", topics_url);

    let comment = HabrCommentsProvider::get_comment(topics_url);
    println!("Random comment: [{}]", comment);

    let message_publisher = TwitterCommentsPublisher::new(twitter_consume_key,
                                                          twitter_consume_secret, twitter_access_key,
                                                          twitter_access_secret);
    let publish_result = message_publisher.publish_comment(comment);

    match publish_result {
        Ok(message) => println!("Message was publsihed: [{}]", message),
        Err(error_msg) => panic!("Message publishing failed with error: [{}]", error_msg),
    }
}
