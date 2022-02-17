#![allow(unused)]

use aggregator::{NewsArticle, Summary, Tweet};
use std::fmt::{Debug, Display};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Water repair!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("New water things happenning in Pittsburg."),
    };

    println!("1 new tweet: {}", tweet.summarize());
    aggregator::notify(&article);
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    5
}

fn same_some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}
