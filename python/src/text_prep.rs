use std::collections::HashSet;

use pyo3::prelude::*;
use ftnt::text_prep::prep as prep_rust;
use ftnt::tokenizer::tweet_tokenize;

#[pyfunction]
#[pyo3(text_signature="(text, encoding, remove_unencodable_char, to_lower, strip_accents, reduce_len, filters, emojismentions, hashtags, urls, digits, puncts, emails, html_tags)")]
pub fn prep(
    text: String,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>,
    filters: Option<HashSet<&str>>,
    emojis: Option<&str>,
    mentions: Option<&str>,
    hashtags: Option<&str>,
    urls: Option<&str>,
    digits: Option<&str>,
    puncts: Option<&str>,
    emails: Option<&str>,
    html_tags: Option<&str>,
)-> String {
    prep_rust(text, encoding, remove_unencodable_char, to_lower, strip_accents, reduce_len, Some(tweet_tokenize), filters, emojis, mentions, hashtags, urls, digits, puncts, emails, html_tags)
}