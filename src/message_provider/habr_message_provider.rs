extern crate rand;

use core::borrow::{Borrow, BorrowMut};

use rand::seq::SliceRandom;
use rand::thread_rng;
use sanitize_html::rules::predefined::RELAXED;
use sanitize_html::sanitize_str;
use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;

use super::abstraction::CommentProvider;
use self::rand::Rng;

pub struct HabrCommentsProvider;

const COMMENTS_BLOCK_SELECTOR_CLASS: &'static str = ".post-stats__comments-link";
const COMMENTS_COUNTER_SELECTOR_CLASS: &'static str = ".post-stats__comments-count";
const COMMENT_SELECTOR_CLASS: &'static str = ".comment__message";
const MIN_COMMENTS_NUM: &'static usize = &10;
const COMMENT_LEN_CHARS: &'static usize = &280;

impl CommentProvider for HabrCommentsProvider {
    fn get_comment(url: &String) -> Option<String> {
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
            reqwest::blocking::get(url).unwrap().text().expect(format!("Unable to get HTML page: [{}]", url).borrow())
        }

        fn sanitize_html(html: &str) -> String {
            sanitize_str(&RELAXED, html).expect("Unable to sanitize comment_str")
        }

        let target_page = url.to_owned() + "/page" + thread_rng().gen_range(1, 9).to_string().borrow();

        println!(
            "Topics URL: [{}]", target_page
        );

        let topics_html_page = get_page_html(&target_page);
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

        let text_comments: Vec<String> = comments
            .iter_mut()
            .map(|c| sanitize_html(c.inner_html().borrow_mut()))
            .map(|comment| {
                comment
                    .replace("<br>", "\n")
                    .replace("<blockquote>", "»")
                    .replace("</blockquote>", "")
                    .replace("<b>", "**")
                    .replace("</b>", "**")
                    .replace("<i>", "*")
                    .replace("</i>", "*")
                    .replace("<s>", "--")
                    .replace("</s>", "--")
                    .replace("<p>", "")
                    .replace("</p>", "\n")
            })
            .filter(|c| { !c.contains("<code ") && !c.contains("<pre>") && !c.contains("<a ") && !c.contains("<img ") })
            .filter(|c| c.len() <= *COMMENT_LEN_CHARS)
            .collect();

        println!("Comments available: {}", text_comments.len());

        text_comments.first().cloned()
    }
}
