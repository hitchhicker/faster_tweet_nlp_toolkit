#![allow(dead_code, unused)]
use std::{ops::{Index, IndexMut}, collections::HashSet, borrow::Borrow, char::REPLACEMENT_CHARACTER};

use itertools::Itertools;
use lazy_static::__Deref;
use regex::Regex;
use encoding_rs::{self, REPLACEMENT};

use crate::{prep::token::{Token, Action}, utils::{strip_accents_unicode, remove_variation_selectors}};

use crate::prep::tokenizer::tweet_tokenize;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ParsedText {
    pub tokens: Vec<Token>,
    pub split: String,
    pub value: Option<String>,
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
        for token in &self.tokens {
            println!("{}", &token.value);
        }
        for token in &mut self.tokens {
            for action in [
                &Action{action_name: mentions_action.map(|s| s.to_string()), action_condition: "is_mention".to_owned()},
                &Action{action_name: hashtags_action.map(|s| s.to_string()), action_condition: "is_hashtag".to_owned()},
                &Action{action_name: urls_action.map(|s| s.to_string()), action_condition: "is_url".to_owned()},
                &Action{action_name: digits_action.map(|s| s.to_string()), action_condition: "is_digit".to_owned()},
                &Action{action_name: emojis_action.map(|s| s.to_string()), action_condition: "is_emoji".to_owned()},
                &Action{action_name: emoticons_action.map(|s| s.to_string()), action_condition: "is_emoticon".to_owned()},
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
        let re = Regex::new(r"\s+").unwrap();
        let result = re.replace_all(text, " ");
        self.value = Some(result.trim().to_string());
    }

    pub fn value(&mut self) -> &str {
        if self.value.is_none() {
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

pub fn _parse_text(
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
        if let Some(encoding_) = encoding_rs::Encoding::for_label(encoding.unwrap().as_bytes()) {
            let (result, _encoding, _errors) = encoding_.encode(&text);
            text = _encoding.decode(&result).0.to_string();
        }
        text = if remove_unencodable_char.unwrap_or(false) {
            text.replace(REPLACEMENT_CHARACTER, "")
        } else {
            let pattern:Regex = Regex::new(&format!(r#"{}{{2,}}$"#, REPLACEMENT_CHARACTER)).unwrap();
            pattern.replace_all(&text, REPLACEMENT_CHARACTER.to_string()).to_string()
        };
    }
    if to_lower.unwrap_or(true) {
        text = text.to_lowercase();
    }
    if strip_accents.unwrap_or(false) {
        text = strip_accents_unicode(&text);
    }
     if reduce_len.unwrap_or(false) {
        text = reduce_lengthening(&text);
    }
    text = remove_variation_selectors(&text);

    let pattern:Regex = Regex::new(r#"([^ ])(https?://)"#).unwrap();
    text = String::from(pattern.replace_all(&text, "$1 $2"));

    let pattern:Regex = Regex::new(r#"(?:P<x>\w+)\?(?:P<y>\w+)"#).unwrap();
    text = String::from(pattern.replace_all(&text, "$x'$y"));

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
    return _parse_text(clean_text, tokenizer, filters, emojis, mentions, hashtags, urls, digits, emoticons, puncts, emails, html_tags, stop_words)
}

pub fn reduce_lengthening(text: &str) -> String {
    let pattern:Regex = Regex::new(r#"(?:P<x>.)\1{2,3}"#).unwrap();
    return String::from(pattern.replace_all(text, "$x"))
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
        assert_eq!(parsed_text.value(), "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries");
        assert_eq!(parsed_text.mentions(), vec![String::from("@nlp")]);
        assert_eq!(parsed_text.emojis(), vec![String::from("😰")]);
        assert_eq!(parsed_text.digits(), vec![String::from("123")]);
        assert_eq!(parsed_text.emails(), vec![String::from("tutu@gmail.com")]);
        assert_eq!(parsed_text.hashtags(), vec![String::from("davidlynch"), String::from("tvseries")]);
        assert_eq!(parsed_text.urls(), vec![String::from("https://www.google.fr")]);
        assert_eq!(parsed_text.len(), 18);

        // test get value by index and set value by index
        assert_eq!(parsed_text[1].value, "c'est");
        parsed_text[1].value = "cest".to_owned();
        assert_eq!(parsed_text[1].value, "cest");

        // test post-processing
        parsed_text.value = Some(" <p> c'est </p> @nlp     https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries".to_owned());
        parsed_text.post_process();
        assert_eq!(parsed_text.value(), "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries");

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
            Some("remove"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
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
            Some(url_action),
            None,
            None,
            None,
            None,
            None,
            None
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
            None
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
            None
        );
        assert_eq!(parsed_text.value(), "@abc :joy: #hashtag");
    }

    #[rstest]
    #[case(None, None, None, None, None, None, None, None, None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(Some("tag"), None, None, None, None, None, None, None, None, None, "<p> c'est </p> <MENTION> https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(Some("remove"), None, None, None, None, None, None, None, None, None, "<p> c'est </p> https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(None, None, Some("tag"), None, None, None, None, None, None, None, "<p> c'est </p> @nlp <URL> cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(None, None, Some("remove"), None, None, None, None, None, None, None, "<p> c'est </p> @nlp cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, Some("tag"), None, None, None, None, None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com <DIGIT> ! #davidlynch #tvseries")]
    #[case(None, None, None, None, Some("tag"), None, None, None, None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait <EMOJI> for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, Some("remove"), None , None, None, None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, Some("demojize"), None, None, None, None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait :cold_sweat: for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, Some("tag"), None, None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 <PUNCT> #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, Some("remove"), None, None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, None, Some("tag"), None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of <EMAIL> 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, None, Some("remove"), None, None, "<p> c'est </p> @nlp https://www.google.fr cant wait 😰 for the new season of 123 ! #davidlynch #tvseries")]
    #[case(None, None, None, None, None, None, None, None, Some("remove"), None, "c'est @nlp https://www.google.fr cant wait 😰 for the new season of tutu@gmail.com 123 ! #davidlynch #tvseries")]
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
        #[case] stop_words_action: Option<&str>,
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
            stop_words_action,
        );
        assert_eq!(parsed_text.value(), expected_value);
    }

    // TODO: support weibo_tokenize
    // #[test]
    // fn test_text_parser_with_weibo_token_class() {
    //     //
    //     let mut parsed_text = parse_text(
    //         String::from("@招商银行 我只是想#改个电话号码#而已。"),
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         weibo_tokenize,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None,
    //         None
    //     );
    // }
}