#![allow(dead_code, unused)]
use std::ops::{Index, IndexMut};

use itertools::Itertools;
use regex::Regex;

use crate::prep::token::{Token, Action};

pub struct ParsedText<'a> {
    tokens: Vec<Token<'a>>,
    split: &'a str,
    value: Option<String>,
}

impl <'a> ParsedText <'a>{
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    pub fn process(
        &mut self,
        mentions_action: Option<String>,
        hashtags_action: Option<String>,
        urls_action: Option<String>,
        digits_action: Option<String>,
        emojis_action: Option<String>,
        emoticons_action: Option<String>,
        puncts_action: Option<String>,
        emails_action: Option<String>,
        html_tags_action: Option<String>,
        stop_words_action: Option<String>,
    ) -> () {
        for token in &mut self.tokens {
            for action in [
                &Action{action_name: Some("mentions_action"), action_condition: "is_mention"},
                &Action{action_name: Some("hashtags_action"), action_condition: "is_hashtag"},
                &Action{action_name: Some("urls_action"), action_condition: "is_url"},
                &Action{action_name: Some("digits_action"), action_condition: "is_digit"},
                &Action{action_name: Some("emojis_action"), action_condition: "is_emoji"},
                &Action{action_name: Some("emoticons_action"), action_condition: "is_emoticon"},
                &Action{action_name: Some("puncts_action"), action_condition: "is_punct"},
                &Action{action_name: Some("emails_action"), action_condition: "is_email"},
                &Action{action_name: Some("html_tags_action"), action_condition: "is_html_tag"},
            ] {
                if token.do_action(action) {
                    break;
                }
            }
        }
        self.tokens = self.tokens.iter().filter(|token| token.value.len() > 0).map(|x|*x).collect::<Vec<_>>()
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
            self.value = Some(String::from(self.tokens.iter().join(self.split)))
        }
        return &self.value.as_ref().unwrap()
    }

}

impl <'a> Index<usize> for ParsedText<'a>{
    type Output = Token<'a>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.tokens[i]
    }
}

impl <'a> IndexMut<usize> for ParsedText<'a> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.tokens[i]
    }
}

pub fn parse_text() -> () {
}