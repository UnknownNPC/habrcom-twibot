extern crate rand;

use htmlescape::decode_html;
use rand::seq::SliceRandom;
use rand::thread_rng;
use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;

use super::abstraction::CommentProvider;

pub struct HabrCommentsProvider;

impl CommentProvider for HabrCommentsProvider {
    fn get_comment(url: String) -> String {
        fn shuffle_list<T>(elements: &mut Vec<T>) {
            let mut rng = thread_rng();
            elements.shuffle(&mut rng);
        }

        fn find_comments_link<'a>(comments_links: &'a Vec<ElementRef>) -> Option<&'a ElementRef<'a>> {
            let comments_counter_selector = Selector::parse(".post-stats__comments-count")
                .expect("Unable to form selector for comments counters");

            comments_links.iter().find(|c| {
                let comments_counter_element: ElementRef = c
                    .select(&comments_counter_selector)
                    .into_iter()
                    .next()
                    .unwrap();
                let comments_counter_parse_res =
                    comments_counter_element.inner_html().parse::<i32>();

                match comments_counter_parse_res {
                    Ok(comments_counter_value) => {
                        if comments_counter_value > 0 {
                            true
                        } else {
                            false
                        }
                    }
                    Err(_e) => false,
                }
            })
        }

        fn get_page_html(url: &str) -> String {
            reqwest::blocking::get(url)
                .unwrap()
                .text()
                .unwrap()
        }

        let topics_html_page = get_page_html(&url);
        let topics_html_document = Html::parse_document(&topics_html_page);

        let comments_block_selector = Selector::parse(".post-stats__comments-link")
            .expect("Unable to form selector for comments info");

        let mut comments_links = topics_html_document
            .select(&comments_block_selector)
            .into_iter()
            .collect();

        shuffle_list(&mut comments_links);

        let comment_link_with_non_zero_counter = find_comments_link(&comments_links)
            .expect("All topics are without comments. Nothing TODO");

        let topic_url = comment_link_with_non_zero_counter
            .value()
            .attr("href")
            .unwrap();

        println!("Topic URL: [{}] from topics size: [{}]", topic_url, comments_links.len());

        let topic_html_page = get_page_html(&topic_url);
        let topic_html_document = Html::parse_document(&topic_html_page);
        let comment_selector = Selector::parse(".comment__message")
            .expect("Unable to form selector for comments on topic page");
        let mut comments = topic_html_document
            .select(&comment_selector)
            .into_iter()
            .collect();
        shuffle_list(&mut comments);
        let comment = comments.first().expect("Topic hasn't comments");

        println!("RAW comment: [{}] from comments size: [{}]", comment.inner_html(), comments.len());

        decode_html(comment.inner_html().as_str()).expect("Unable to decode html")
    }
}

