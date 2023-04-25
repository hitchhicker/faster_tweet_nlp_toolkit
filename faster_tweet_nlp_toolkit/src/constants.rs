#![allow(dead_code, unused)]
use std::{collections::{HashMap}, vec};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EMOJI_TAG: &'static str = "<EMOJI>";
    pub static ref MENTION_TAG: &'static str = "<MENTION>";
    pub static ref HASHTAG_TAG: &'static str = "<HASHTAG>";
    pub static ref UNENCODABLE_CHAR: &'static str = "�";
    pub static ref URL_TAG: &'static str = "<URL>";
    pub static ref DIGIT_TAG: &'static str = "<DIGIT>";
    pub static ref EMOTICON_TAG: &'static str = "<EMOTICON>";
    pub static ref PUNCTUATION_TAG: &'static str = "<PUNCT>";
    pub static ref EMAIL_TAG: &'static str = "<EMAIL>";
    pub static ref VARIATION_SELECTORS: [&'static str; 16] = [
        "\u{fe00}",
        "\u{fe01}",
        "\u{fe02}",
        "\u{fe03}",
        "\u{fe04}",
        "\u{fe05}",
        "\u{fe06}",
        "\u{fe07}",
        "\u{fe08}",
        "\u{fe09}",
        "\u{fe0a}",
        "\u{fe0b}",
        "\u{fe0c}",
        "\u{fe0d}",
        "\u{fe0e}",
        "\u{fe0f}",
    ];
    pub static ref REPLACE_MAPPINGS: HashMap<&'static str, &'static str> = HashMap::from([
        ("is_mention", *MENTION_TAG),
        ("is_hashtag", *HASHTAG_TAG),
        ("is_url", *URL_TAG),
        ("is_digit", *DIGIT_TAG),
        ("is_emoji", *EMOJI_TAG),
        ("is_emoticon", *EMOTICON_TAG),
        ("is_punct", *PUNCTUATION_TAG),
        ("is_email", *EMAIL_TAG),
    ]);
    pub static ref ACTION_MAPPING: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
        ("is_mention", vec!["remove", "tag"]),
        ("is_hashtag", vec!["remove", "tag"]),
        ("is_url", vec!["remove", "tag"]),
        ("is_digit", vec!["remove", "tag"]),
        ("is_emoji", vec!["remove", "tag", "demojize", "emojize"]),
        ("is_emoticon", vec!["remove", "tag"]),
        ("is_punct", vec!["remove", "tag"]),
        ("is_email", vec!["remove", "tag"]),
        ("is_html_tag", vec!["remove"]),
        ("is_stop_word", vec!["remove"]),
    ]);
}