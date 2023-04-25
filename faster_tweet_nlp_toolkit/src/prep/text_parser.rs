#![allow(dead_code, unused)]
use std::{ops::{Index, IndexMut}, collections::HashSet, borrow::Borrow, char::REPLACEMENT_CHARACTER};
use itertools::Itertools;
use lazy_static::{__Deref, lazy_static};
use pcre2::bytes::{Regex, Match};
use encoding_rs::{self, REPLACEMENT};

use crate::prep::token::{Token, Action};
use crate::prep::utils::{strip_accents_unicode, remove_variation_selectors, preprocess_url};
use crate::prep::tokenizer::tweet_tokenize;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ParsedText {
    pub tokens: Vec<Token>,
    pub split: String,
    pub value: Option<String>,
}

impl ParsedText{
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
        emoticons_actions: Option<&str>,
        puncts_action: Option<&str>,
        emails_action: Option<&str>,
        html_tags_action: Option<&str>,
    ) -> () {
        for token in &mut self.tokens {
            for action in [
                &Action{action_name: mentions_action.map(|s| s.to_string()), action_condition: "is_mention".to_owned()},
                &Action{action_name: hashtags_action.map(|s| s.to_string()), action_condition: "is_hashtag".to_owned()},
                &Action{action_name: urls_action.map(|s| s.to_string()), action_condition: "is_url".to_owned()},
                &Action{action_name: digits_action.map(|s| s.to_string()), action_condition: "is_digit".to_owned()},
                &Action{action_name: emojis_action.map(|s| s.to_string()), action_condition: "is_emoji".to_owned()},
                &Action{action_name: emoticons_actions.map(|s| s.to_string()), action_condition: "is_emoticon".to_owned()},
                &Action{action_name: puncts_action.map(|s| s.to_string()), action_condition: "is_punct".to_owned()},
                &Action{action_name: emails_action.map(|s| s.to_string()), action_condition: "is_email".to_owned()},
                &Action{action_name: html_tags_action.map(|s| s.to_string()), action_condition: "is_html_tag".to_owned()},
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
        lazy_static! {
            static ref CONTINUOUS_SPACES: Regex = Regex::new(r"\s+").unwrap();
        }
        let result = CONTINUOUS_SPACES.replace_all(text.as_bytes(), " ".as_bytes());
        self.value = Some(String::from_utf8(result.to_vec()).unwrap().trim().to_string());
    }

    pub fn value(&mut self) -> &str {
        if self.value.is_none() {
            self.value = Some(String::from(self.tokens.iter().join(&self.split)))
        }
        return &self.value.as_ref().unwrap()
    }

    pub fn hashtags(&self) -> Vec<String> {
        return self.tokens.iter().map(|x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn mentions(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_mention()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn emojis(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_emoji()).map(
                |x| x.clone()).map(|x| String::from(x.value)).collect::<Vec<String>>()
    }

    pub fn emoticons(&self) -> Vec<String> {
        return self.tokens.iter().filter(
            |token| token.is_emoticon()).map(
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

fn _parse_text(
    clean_text: String,
    tokenizer: Option<fn(String) -> Vec<Token>>,
    filters: Option<HashSet<&str>>,
    emojis: Option<&str>,
    emoticons_actions: Option<&str>,
    mentions: Option<&str>,
    hashtags: Option<&str>,
    urls: Option<&str>,
    digits: Option<&str>,
    puncts: Option<&str>,
    emails: Option<&str>,
    html_tags: Option<&str>,
) -> ParsedText {
    let filters = filters.unwrap_or(HashSet::new());
    let mut parsed_text = ParsedText {tokens: tokenizer
        .unwrap_or(tweet_tokenize)(clean_text)
        .iter()
        .filter(|token| !filters.contains(token.value.as_str()))
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
        emoticons_actions,
        puncts,
        emails,
        html_tags,
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
        if let Some(encoding_) = encoding_rs::Encoding::for_label(encoding.unwrap().as_bytes()) {
            let (result, _encoding, _errors) = encoding_.encode(&text);
            text = _encoding.decode(&result).0.to_string();
        }
        text = if remove_unencodable_char.unwrap_or(false) {
            text.replace(REPLACEMENT_CHARACTER, "")
        } else {
            lazy_static! {
                static ref RE: Regex = Regex::new(&format!(r#"{}{{2,}}$"#, REPLACEMENT_CHARACTER)).unwrap();
            }
            let pattern: &Regex = &RE;
            String::from_utf8(pattern.replace_all(text.as_bytes(), REPLACEMENT_CHARACTER.to_string().as_bytes()).to_vec()).unwrap()
        };
    }
    if to_lower.unwrap_or(true) {
        text = text.to_lowercase();
    }
    if strip_accents.unwrap_or(false) {
        text = strip_accents_unicode(&text).to_string();
    }
    if reduce_len.unwrap_or(false) {
        text = reduce_lengthening(&text);
    }
    text = remove_variation_selectors(&text);

    text = preprocess_url(&text);

    // c?est -> c'est
    lazy_static! {
        static ref REPEAT_RE: Regex = Regex::new(r#"(?:P<x>\w+)\?(?:P<y>\w+)"#).unwrap();
    }
    let pattern: &Regex = &REPEAT_RE;
    text = String::from_utf8(pattern.replace_all(text.as_bytes(), "$x'$y".as_bytes()).to_vec()).unwrap();

    text = html_escape::decode_html_entities(&text).to_string();
    return text
}



pub fn parse_text(
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
) -> ParsedText{
    let clean_text = preprocess_text(text, encoding, remove_unencodable_char, to_lower, strip_accents, reduce_len);
    _parse_text(clean_text, tokenizer, filters, emojis, emoticons, mentions, hashtags, urls, digits, puncts, emails, html_tags)
}

pub fn reduce_lengthening(text: &str) -> String {
    lazy_static! {
        static ref LENGTHENING_RE: Regex = Regex::new(r#"(.)\1{2,}"#).unwrap();
    }
    let pattern: &Regex = &LENGTHENING_RE;
    let mut res: String = text.to_string().clone();
    for result in pattern.captures_iter(&text.as_bytes()) {
        let captures = &result.unwrap();
        let replace_from = &text[captures.get(0).unwrap().start()..captures.get(0).unwrap().end()];
        let replace_to = &text[captures.get(1).unwrap().start()..captures.get(1).unwrap().end()].repeat(3);
        res = res.replace(replace_from, replace_to);
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use std::{hash::Hash, vec};

    use rstest::rstest;

    use super::*;

    fn _get_mock_parsed_text() -> ParsedText{
        let tokens = vec![
                Token {value: String::from("<p>")},
                Token {value: String::from("c\'est")},
                Token {value: String::from("</p>")},
                Token {value: String::from("@nlp")},
                Token {value: String::from("https://www.google.fr")},
                Token {value: String::from("cant")},
                Token {value: String::from("wait")},
                Token {value: String::from("😰")},
                Token {value: String::from("for")},
                Token {value: String::from("the")},
                Token {value: String::from("new")},
                Token {value: String::from("season")},
                Token {value: String::from("of")},
                Token {value: String::from("tutu@gmail.com")},
                Token {value: String::from(r"\(^o^)/")},
                Token {value: String::from("123")},
                Token {value: String::from("!")},
                Token {value: String::from("#davidlynch")},
                Token {value: String::from("#tvseries")},
            ];
            return ParsedText {tokens: tokens, split: " ".to_string(), value: None };
        }

    #[test]
    fn test_text_parser_mentions() {
        let mut parsed_text = _get_mock_parsed_text();
        assert_eq!(parsed_text.value(), r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries");
        assert_eq!(parsed_text.mentions(), vec![String::from("@nlp")]);
        assert_eq!(parsed_text.emojis(), vec![String::from("😰")]);
        assert_eq!(parsed_text.emoticons(), vec![String::from(r"\(^o^)/")]);
        assert_eq!(parsed_text.digits(), vec![String::from("123")]);
        assert_eq!(parsed_text.emails(), vec![String::from("tutu@gmail.com")]);
        // assert_eq!(parsed_text.hashtags(), vec![String::from("davidlynch"), String::from("tvseries")]);
        assert_eq!(parsed_text.urls(), vec![String::from("https://www.google.fr")]);
        assert_eq!(parsed_text.len(), 19);

        // test get value by index and set value by index
        assert_eq!(parsed_text[1].value, "c'est");
        parsed_text[1].value = "cest".to_owned();
        assert_eq!(parsed_text[1].value, "cest");

        // test post-processing
        parsed_text.value = Some(r" <p> c'est </p> @nlp     https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries".to_owned());
        parsed_text.post_process();
        assert_eq!(parsed_text.value(), r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries");

    }

    #[test]
    fn test_text_parser_when_there_is_emoji() {
        let mut parsed_text = parse_text(
            String::from("July @AlraashidS @shalsaeedi_ @asaldhferi @Fa3ix_ @iiBeba_ @_hajaraljble ❤️"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("remove"),
            None,
            Some("remove"),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(parsed_text.value(), "july");
    }

    #[rstest]
    #[case("remove", "asylum seeker :")]
    #[case("tag", "asylum seeker : <URL>")]
    fn test_text_parser_with_attached_url(#[case] url_action: &str, #[case] expected: &str) {
        let mut parsed_text = parse_text(
            String::from("asylum seeker:http://t.co/skU8zM7Slh"),
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
            Some(url_action),
            None,
            None,
            None,
            None,
        );
        assert_eq!(parsed_text.value(), expected);
    }

    #[test]
    fn test_text_parser_with_emoji_string() {
        let mut parsed_text = parse_text(
            String::from("@abc:joy:#hashtag"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("emojize"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(parsed_text.value(), "@abc 😂 #hashtag");
    }

    #[test]
    fn test_text_parser_with_demojize() {
        let mut parsed_text = parse_text(
            String::from("@abc😂#hashtag"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("demojize"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(parsed_text.value(), "@abc :joy: #hashtag");
    }

    #[rstest]
    #[case(None, None, None, None, None, None, None, None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(Some("tag"), None, None, None, None, None, None, None, None, r"<p> c'est </p> <MENTION> https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(Some("remove"), None, None, None, None, None, None, None, None, r"<p> c'est </p> https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, Some("tag"), None, None, None, None, None, None, r"<p> c'est </p> @nlp <URL> cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, Some("remove"), None, None, None, None, None, None, r"<p> c'est </p> @nlp cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, Some("tag"), None, None, None, None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ <DIGIT> ! #davidlynch #tvseries")]
    #[case(None, None, None, None, Some("tag"), None, None, None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait <EMOJI> for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, Some("remove") , None, None, None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, Some("demojize"), None, None, None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait :cold_sweat: for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, Some("tag"), None, None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com <EMOTICON> 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, Some("remove"), None, None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, Some("tag"), None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 <PUNCT> #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, Some("remove"), None, None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, None, Some("tag"), None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of <EMAIL> \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None,  None, Some("remove"), None, r"<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of \(^o^)/ 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, None, None, Some("remove"), r"c'est @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com \(^o^)/ 123 ! #davidlynch #tvseries")]
    fn test_text_parser_post_process(
        #[case] mentions_action: Option<&str>,
        #[case] hashtags_action: Option<&str>,
        #[case] urls_action: Option<&str>,
        #[case] digits_action: Option<&str>,
        #[case] emojis_action: Option<&str>,
        #[case] emoticons_action: Option<&str>,
        #[case] puncts_action: Option<&str>,
        #[case] emails_action: Option<&str>,
        #[case] html_tags_action: Option<&str>,
        #[case] expected_value: &str,
    ) {
        let mut parsed_text = _get_mock_parsed_text();
        parsed_text.process(
            mentions_action,
            hashtags_action,
            urls_action,
            digits_action,
            emojis_action,
            emoticons_action,
            puncts_action,
            emails_action,
            html_tags_action,
        );
        assert_eq!(parsed_text.value(), expected_value);
    }
    #[rstest]
    #[case("This is waaaaayyyy too much for you!!!!!!", "This is waaayyy too much for you!!!")]
    fn test_reduce_lengthening(#[case] text: &str, #[case] expected: &str) {
        assert_eq!(reduce_lengthening(text), expected);
    }
}