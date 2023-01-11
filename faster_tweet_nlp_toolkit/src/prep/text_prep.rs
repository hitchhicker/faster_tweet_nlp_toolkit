use std::collections::HashSet;

use super::{text_parser::parse_text, token::Token};

pub fn prep(
    text: String,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>,
    tokenizer: Option<fn(String) -> Vec<Token>>,
    filters: Option<HashSet<&str>>,
    emojis: Option<&str>,
    emoticons: Option<&str>,
    mentions: Option<&str>,
    hashtags: Option<&str>,
    urls: Option<&str>,
    digits: Option<&str>,
    puncts: Option<&str>,
    emails: Option<&str>,
    html_tags: Option<&str>,
) -> String {
    return parse_text(
        text,
        encoding,
        remove_unencodable_char,
        to_lower,
        strip_accents,
        reduce_len,
        tokenizer,
        filters,
        emojis,
        emoticons,
        mentions,
        hashtags,
        urls,
        digits,
        puncts,
        emails,
        html_tags,
    ).value().to_string()
}