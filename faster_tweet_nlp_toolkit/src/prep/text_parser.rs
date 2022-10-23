#![allow(dead_code, unused)]
use std::{ops::{Index, IndexMut}, collections::HashSet};

use itertools::Itertools;
use regex::Regex;

use crate::{prep::token::{Token, Action}, utils::{strip_accents_unicode, remove_variation_selectors}};

use crate::prep::tokenizer::tweet_tokenize;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ParsedText {
    tokens: Vec<Token>,
    split: String,
    value: Option<String>,
}

impl  ParsedText{
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    pub fn process(
        &mut self,
        mentions_action: Option<&str>,
        hashtags_action: Option<&str>,
        urls_action: Option<&str>,
        digits_action: Option<&str>,
        emojis_action: Option<&str>,
        emoticons_action: Option<&str>,
        puncts_action: Option<&str>,
        emails_action: Option<&str>,
        html_tags_action: Option<&str>,
        stop_words_action: Option<&str>,
    ) -> () {
        for token in &mut self.tokens {
            for action in [
                &Action{action_name: Some("mentions_action".to_owned()), action_condition: "is_mention".to_owned()},
                &Action{action_name: Some("hashtags_action".to_owned()), action_condition: "is_hashtag".to_owned()},
                &Action{action_name: Some("urls_action".to_owned()), action_condition: "is_url".to_owned()},
                &Action{action_name: Some("digits_action".to_owned()), action_condition: "is_digit".to_owned()},
                &Action{action_name: Some("emojis_action".to_owned()), action_condition: "is_emoji".to_owned()},
                &Action{action_name: Some("emoticons_action".to_owned()), action_condition: "is_emoticon".to_owned()},
                &Action{action_name: Some("puncts_action".to_owned()), action_condition: "is_punct".to_owned()},
                &Action{action_name: Some("emails_action".to_owned()), action_condition: "is_email".to_owned()},
                &Action{action_name: Some("html_tags_action".to_owned()), action_condition: "is_html_tag".to_owned()},
            ] {
                if token.do_action(action) {
                    break;
                }
            }
        }
        self.tokens = self.tokens
        .iter()
        .filter(|token| token.value.len() > 0)
        .map(|x| Token{ value: (*x.value).to_string()})
        .collect::<Vec<_>>()
    }

    pub fn post_process(&mut self) -> () {
        let text = self.value();
        let re = Regex::new(r"\s+").unwrap();
        let result = re.replace_all(text, " ");
        self.value = Some(text.trim().to_string());
    }

    pub fn value(&mut self) -> &str {
        if self.value.is_none() {
            let names = ["firstName", "lastName"];
            let joined = names.join(", ");
            self.value = Some(String::from(self.tokens.iter().join(&self.split)))
        }
        return &self.value.as_ref().unwrap()
    }

    pub fn hashtags(&self) -> Vec<&str> {
        return self.tokens.iter().filter(
            |token| token.is_hashtag()).map(
                |x| x.value.strip_prefix("#").unwrap()).collect::<Vec<&str>>()
    }

    pub fn mentions(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_mention()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn emoticons(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_emoticon()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn emojis(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_emoji()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn digits(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_digit()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn emails(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_email()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn urls(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_url()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }
}

impl Index<usize> for ParsedText{
    type Output = Token;
    fn index(&self, i: usize) -> &Self::Output {
        &self.tokens[i]
    }
}

impl IndexMut<usize> for ParsedText{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.tokens[i]
    }
}

pub fn parse_text(
    clean_text: String,
    tokenizer: Option<fn(String) -> Vec<Token>>,
    filters: Option<HashSet<&str>>,
    emojis: Option<&str>,
    mentions: Option<&str>,
    hashtags: Option<&str>,
    urls: Option<&str>,
    digits: Option<&str>,
    emoticons: Option<&str>,
    puncts: Option<&str>,
    emails: Option<&str>,
    html_tags: Option<&str>,
    stop_words: Option<&str>,
) -> ParsedText {
    let filters = filters.unwrap_or(HashSet::new());
    let mut parsed_text = ParsedText {tokens: tokenizer
        .unwrap_or(tweet_tokenize)(clean_text)
        .iter()
        .filter(|token| filters.contains(token.value.as_str()))
        .map(|x| Token{ value: (*x.value).to_string()})
        .collect::<_>(),
        split: String::from(" "),
        value: None
    };
    parsed_text.process(
        mentions,
        hashtags,
        urls,
        digits,
        emojis,
        emoticons,
        puncts,
        emails,
        html_tags,
        stop_words,
    );
    return parsed_text
}


pub fn preprocess_text(
    mut text: String,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>
) -> String {
    if encoding.is_some() {
        /*
        TODO:
        if encoding is not None:
            text = text.encode(encoding, "surrogatepass").decode(encoding, "replace")
            if remove_unencodable_char:
                text = text.replace(UNENCODABLE_CHAR, " ")
            else:  # change any sequence of unknown characters to a single one
                text = re.sub(UNENCODABLE_CHAR + "{2,}", UNENCODABLE_CHAR, text)
        */
    }
    if to_lower.unwrap_or(false) {
        text = text.to_lowercase();
    }
    if strip_accents.unwrap_or(false) {
        text = strip_accents_unicode(&text);
    }
     if reduce_len.unwrap_or(false) {
        text = reduce_lengthening(&text);
    }
    text = remove_variation_selectors(&text);

    let pattern:Regex = Regex::new(r#"(?:P<x>[^ ])(?:P<y>https?://)"#).unwrap();
    text = String::from(pattern.replace_all(&text, "$x $y"));

    let pattern:Regex = Regex::new(r#"(?:P<x>\w+)\?(?:P<y>\w+)"#).unwrap();
    text = String::from(pattern.replace_all(&text, "$x'$y"));
    /*
    TODO:
    text = html.unescape(text)  # &pound;100 -> £100
    */
    return text
}

pub fn myfunc(
    text: String,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>,
    tokenizer: Option<fn(String) -> Vec<Token>>,
    filters: Option<HashSet<&str>>,
    emojis: Option<&str>,
    mentions: Option<&str>,
    hashtags: Option<&str>,
    urls: Option<&str>,
    digits: Option<&str>,
    emoticons: Option<&str>,
    puncts: Option<&str>,
    emails: Option<&str>,
    html_tags: Option<&str>,
    stop_words: Option<&str>,
) -> ParsedText{
    let clean_text = preprocess_text(text, encoding, remove_unencodable_char, to_lower, strip_accents, reduce_len);
    return parse_text(clean_text, tokenizer, filters, emojis, mentions, hashtags, urls, digits, emoticons, puncts, emails, html_tags, stop_words)

}

pub fn reduce_lengthening(text: &str) -> String {
    let pattern:Regex = Regex::new(r#"(?:P<x>.)\1{2,3}"#).unwrap();
    return String::from(pattern.replace_all(text, "$x"))
}