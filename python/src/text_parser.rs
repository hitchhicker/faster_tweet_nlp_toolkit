#![allow(dead_code, unused)]
use std::collections::HashSet;

use ftnt::{text_parser::{ParsedText, _parse_text}, token::{Action, Token}, tokenizer::tweet_tokenize};
use ftnt::text_parser::parse_text as parse_text_rust;
use ftnt::text_parser::preprocess_text as preprocess_text_rust;
use pyo3::prelude::*;
use regex::Regex;
use itertools::Itertools;

#[pyclass(module = "faster_tweet_nlp_toolkit", name = "ParsedText")]
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct PyParsedText {
    parsed_text: ParsedText,
}
impl From<ParsedText> for PyParsedText {
    fn from(parsed_text: ParsedText) -> Self {
        Self { parsed_text }
    }
}

#[pymethods]
impl PyParsedText {
    #[getter]
    fn tokens(&self) -> PyResult<Vec<String>> {
        Ok(self.parsed_text.tokens.iter().map(|x|x.to_string()).collect())
    }

    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.parsed_text.tokens.len())
    }

    #[pyo3(text_signature = "(self, mentions_action, hashtags_action, urls_action, digits_action, emojis_action, puncts_action, emails_action, html_tags_action)")]
    pub fn process(
        &mut self,
        mentions_action: Option<&str>,
        hashtags_action: Option<&str>,
        urls_action: Option<&str>,
        digits_action: Option<&str>,
        emojis_action: Option<&str>,
        puncts_action: Option<&str>,
        emails_action: Option<&str>,
        html_tags_action: Option<&str>,
    ) -> () {
        for token in &mut self.parsed_text.tokens {
            for action in [
                &Action{action_name: mentions_action.map(|s| s.to_string()), action_condition: "is_mention".to_owned()},
                &Action{action_name: hashtags_action.map(|s| s.to_string()), action_condition: "is_hashtag".to_owned()},
                &Action{action_name: urls_action.map(|s| s.to_string()), action_condition: "is_url".to_owned()},
                &Action{action_name: digits_action.map(|s| s.to_string()), action_condition: "is_digit".to_owned()},
                &Action{action_name: emojis_action.map(|s| s.to_string()), action_condition: "is_emoji".to_owned()},
                &Action{action_name: puncts_action.map(|s| s.to_string()), action_condition: "is_punct".to_owned()},
                &Action{action_name: emails_action.map(|s| s.to_string()), action_condition: "is_email".to_owned()},
                &Action{action_name: html_tags_action.map(|s| s.to_string()), action_condition: "is_html_tag".to_owned()},
            ] {
                if token.do_action(action) {
                    break;
                }
            }
        }
        self.parsed_text.tokens = self.parsed_text.tokens
        .iter()
        .filter(|token| token.value.len() > 0)
        .map(|x| Token{ value: (*x.value).to_string()})
        .collect::<Vec<_>>()
    }

    pub fn post_process(&mut self) -> () {
        let text = self.value();
        let re = Regex::new(r"\s+").unwrap();
        let result = re.replace_all(text, " ");
        self.parsed_text.value = Some(result.trim().to_string());
    }

    #[getter]
    pub fn value(&mut self) -> &str {
        if self.parsed_text.value.is_none() {
            self.parsed_text.value = Some(String::from(self.parsed_text.tokens.iter().join(&self.parsed_text.split)))
        }
        return &self.parsed_text.value.as_ref().unwrap()
    }

    #[getter]
    pub fn hashtags(&self) -> Vec<&str> {
        return self.parsed_text.tokens.iter().filter(
            |token| token.is_hashtag()).map(
                |x| x.value.strip_prefix("#").unwrap()).collect::<Vec<&str>>()
    }

    #[getter]
    pub fn mentions(&self) -> Vec<String> {
        return self.parsed_text.tokens.iter().filter(
            |token| token.is_mention()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    #[getter]
    pub fn emojis(&self) -> Vec<String> {
        return self.parsed_text.tokens.iter().filter(
            |token| token.is_emoji()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    #[getter]
    pub fn digits(&self) -> Vec<String> {
        return self.parsed_text.tokens.iter().filter(
            |token| token.is_digit()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    #[getter]
    pub fn emails(&self) -> Vec<String> {
        return self.parsed_text.tokens.iter().filter(
            |token| token.is_email()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    #[getter]
    pub fn urls(&self) -> Vec<String> {
        return self.parsed_text.tokens.iter().filter(
            |token| token.is_url()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }
}

#[pyfunction]
#[pyo3(text_signature="(text, encoding, remove_unencodable_char, to_lower, strip_accents, reduce_len, filters, emojis, mentions, hashtags, urls, digits, puncts, emails, html_tags)")]
pub fn parse_text(
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
) -> PyParsedText{
    PyParsedText::from(parse_text_rust(
        text,
        encoding,
        remove_unencodable_char,
        to_lower,
        strip_accents,
        reduce_len,
        Some(tweet_tokenize),
        filters,
        emojis,
        mentions,
        hashtags,
        urls,
        digits,
        puncts,
        emails,
        html_tags
    )).into()
}

#[pyfunction]
#[pyo3(text_signature="(text, encoding, remove_unencodable_char, to_lower, strip_accents, reduce_len)")]
pub fn preprocess_text(
    mut text: String,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>,
) -> String {
    preprocess_text_rust(
        text,
        encoding,
        remove_unencodable_char,
        to_lower,
        strip_accents,
        reduce_len,
    )
}
