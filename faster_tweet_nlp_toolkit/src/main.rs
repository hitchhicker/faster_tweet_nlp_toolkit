pub mod prep;
pub mod constants;
pub mod utils;

use std::{time::Instant};

use faster_tweet_nlp_toolkit::text_parser::parse_text;
use regex::Regex;

fn main() {
    let _start = Instant::now();
    let parsed_text = parse_text(
        String::from("123 @hello #world www.url.com 😰 abc@gmail.com"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    let start = Instant::now();
    let mut i = 0;

    while i < 100 {
        let _a = parsed_text.hashtags();
        i += 1;
    }
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    // let emojit_regex: Regex = Regex::new(r#"(?::\w+:)"#).unwrap();
    // for mat in emojit_regex.find_iter(":http:") {
    //     println!("{:?}", mat);
    // }
    let pattern:Regex = Regex::new(r#"([^ ])(https?://)"#).unwrap();
    let text = "asylum seeker:http://t.co/skU8zM7Slh";
    for mat in pattern.find_iter(text) {
        println!("{:?}", mat);
    }
    let text2 = String::from(pattern.replace_all(&text, "$1 $2"));
    println!("{}", text2)
}
