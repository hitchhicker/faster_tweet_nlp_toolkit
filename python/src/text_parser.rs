#![allow(dead_code, unused)]
use std::collections::HashSet;

use ftnt::{text_parser::{ParsedText, _parse_text}, token::{Action, Token}, tokenizer::tweet_tokenize};
use ftnt::text_parser::parse_text as parse_text_rust;
use ftnt::text_parser::preprocess_text as preprocess_text_rust;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pcre2::bytes::Regex;
use itertools::Itertools;

use crate::token::PyToken;

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
    fn __str__(&mut self) -> PyResult<String>   {
        Ok(format!("{}", self.parsed_text.value()))
    }

    fn __repr__(&mut self) -> PyResult<String>   {
        self.__str__()
    }

    #[getter]
    fn tokens(&self) -> Vec<PyToken> {
        self.parsed_text.tokens.iter().map(|x| PyToken::from(x.clone())).collect()
    }

    pub fn __len__(&mut self) -> PyResult<usize> {
        Ok(self.parsed_text.value().len())
    }

    #[pyo3(text_signature = "(self, mentions_action, hashtags_action, urls_action, digits_action, emojis_action, emoticons_action, puncts_action, emails_action, html_tags_action)")]
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
    ) -> () {
        return self.parsed_text.process(mentions_action, hashtags_action, urls_action, digits_action, emojis_action, emoticons_action, puncts_action, emails_action, html_tags_action)
    }

    pub fn post_process(&mut self) -> () {
        return self.parsed_text.post_process()
    }

    #[getter]
    pub fn value(&mut self) -> &str {
        return self.parsed_text.value()
    }

    #[getter]
    pub fn hashtags(&self) -> Vec<String> {
        return self.parsed_text.hashtags()
    }

    #[getter]
    pub fn mentions(&self) -> Vec<String> {
        return self.parsed_text.mentions()
    }

    #[getter]
    pub fn emojis(&self) -> Vec<String> {
        return self.parsed_text.emojis()
    }

    #[getter]
    pub fn digits(&self) -> Vec<String> {
        return self.parsed_text.digits()
    }

    #[getter]
    pub fn emails(&self) -> Vec<String> {
        return self.parsed_text.emails()
    }

    #[getter]
    pub fn urls(&self) -> Vec<String> {
        return self.parsed_text.urls()
    }
}

#[pyfunction]
#[pyo3(text_signature="(text, encoding, remove_unencodable_char, to_lower, strip_accents, reduce_len, filters, emojis, emoticons, mentions, hashtags, urls, digits, puncts, emails, html_tags)")]
pub fn parse_text(
    text: String,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>,
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
        emoticons,
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
