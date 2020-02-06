extern crate rand;

use core::borrow::BorrowMut;

use rand::seq::SliceRandom;
use rand::thread_rng;
use sanitize_html::rules::predefined::RELAXED;
use sanitize_html::sanitize_str;
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};

use super::abstraction::CommentProvider;

pub struct HabrCommentsProvider;

const COMMENTS_BLOCK_SELECTOR_CLASS: &'static str = ".post-stats__comments-link";
const COMMENTS_COUNTER_SELECTOR_CLASS: &'static str = ".post-stats__comments-count";
const COMMENT_SELECTOR_CLASS: &'static str = ".comment__message";
const MIN_COMMENTS_NUM: &'static usize = &10;
const COMMENT_LEN_CHARS: &'static usize = &280;

impl CommentProvider for HabrCommentsProvider {
    fn get_comment(url: String) -> String {
        fn shuffle_list<T>(elements: &mut Vec<T>) {
            let mut rng = thread_rng();
            elements.shuffle(&mut rng);
        }

        fn find_comments_link<'a>(
            comments_links: &'a Vec<ElementRef>,
        ) -> Option<&'a ElementRef<'a>> {
            let comments_counter_selector = Selector::parse(COMMENTS_COUNTER_SELECTOR_CLASS)
                .expect("Unable to form selector for comments counters");

            comments_links.iter().find(|c| {
                let comments_counter_element: ElementRef = c
                    .select(&comments_counter_selector)
                    .into_iter()
                    .next()
                    .unwrap();
                let comments_counter_parse_res =
                    comments_counter_element.inner_html().parse::<usize>();

                match comments_counter_parse_res {
                    Ok(comments_counter_value) => {
                        if comments_counter_value >= *MIN_COMMENTS_NUM {
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
            reqwest::blocking::get(url).unwrap().text().unwrap()
        }

        fn sanitize_html(mut html: &str) -> String {
            sanitize_str(&RELAXED, html.borrow_mut()).expect("Unable to sanitize comment_str")
        }

        let topics_html_page = get_page_html(&url);
        let topics_html_document = Html::parse_document(&topics_html_page);

        let comments_block_selector = Selector::parse(COMMENTS_BLOCK_SELECTOR_CLASS)
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

        println!(
            "Topic URL: [{}] from topics size: [{}]",
            topic_url,
            comments_links.len()
        );

        let topic_html_page = get_page_html(&topic_url);
        let topic_html_document = Html::parse_document(&topic_html_page);
        let comment_selector = Selector::parse(COMMENT_SELECTOR_CLASS)
            .expect("Unable to form selector for comments on topic page");
        let mut comments = topic_html_document
            .select(&comment_selector)
            .into_iter()
            .collect();
        shuffle_list(&mut comments);
        let comment_str = comments
            .iter_mut()
            .map(|c| sanitize_html(c.inner_html().borrow_mut()))
            .find(|c| c.len() <= *COMMENT_LEN_CHARS)
            .expect("Topic hasn't comments available for tweeter :<");

        println!(
            "RAW comment_str: [{}] from comments size: [{}]",
            comment_str,
            comments.len()
        );

        comment_str
    }
}
