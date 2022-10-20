#![allow(dead_code, unused)]
use std::fmt::Display;
use std::ops::{Index, IndexMut};

use regex::Regex;
use crate::prep::regexes::*;
use crate::constants::*;
use unicode_categories::UnicodeCategories;
use emojis;
use rstest::rstest;


#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Token<'a>{
    pub value: &'a str,
}

impl <'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn _is_punct(value: &str) -> bool {
    value.chars().next().unwrap().is_punctuation()
}

fn _is_emoji_alias(value: &str) -> bool {
    let emoji_opt = emojis::get_by_shortcode(value);
    match emoji_opt {
        Some(_emoji) => true,
        _ => false
    }
}

fn _is_unicode_emoji(value: &str) -> bool {
    let emoji_opt = emojis::get(value);
    match emoji_opt {
        Some(_emoji) => true,
        _ => false
    }
}

/*
Error:
   |
40 |         &self.value.chars().nth(i).unwrap()
   |         ^----------------------------------
   |         ||
   |         |temporary value created here
   |         returns a reference to data owned by the current function
*/
// impl Index<usize> for Token {
//     type Output = char;
//     fn index(&self, i: usize) -> &Self::Output {
//         &self.value.chars().nth(i).unwrap()
//     }
// }

// impl IndexMut<usize> for Token {
//     fn index_mut(&mut self, i: usize) -> &mut Self::Output {
//         &mut self.value.chars().nth(i)
//     }
// }

impl <'a> Token <'a>{
    pub fn set_value(&mut self, new_value: &'a str) {
        self.value = new_value;
    }

    pub fn is_emoji(&self) -> bool {
        _is_unicode_emoji(&self.value) || _is_emoji_alias(&self.value)
    }

    pub fn is_punct(&self) -> bool {
        _is_punct(&self.value)
    }

    pub fn check_flag(&self, pattern: &str) -> bool {
        let compiled_pattern:Regex = Regex::new(&format!(r"^{}$", pattern)).unwrap();
        compiled_pattern.is_match(&self.value)
    }

    pub fn is_hashtag(&self) -> bool {
        !self.check_flag(*NOT_A_HASHTAG) & self.check_flag(*HASHTAG)
    }

    pub fn is_url(&self) -> bool {
        self.check_flag(*URL)
    }

    pub fn is_mention(&self) -> bool {
        self.check_flag(*MENTION)
    }

    pub fn is_emoticon(&self) -> bool {
        self.check_flag(*EMOTICONS)
    }

    pub fn is_digit(&self) -> bool {
        self.check_flag(*DIGIT)
    }

    pub fn is_email(&self) -> bool {
        self.check_flag(*EMAIL)
    }

    pub fn is_html_tag(&self) -> bool {
        self.check_flag(*HTML_TAG)
    }

    pub fn do_action(&mut self, action: &Action) -> bool {
        action.apply(self)
    }
}

pub struct Action<'a> {
    pub action_name: Option<&'a str>,
    pub action_condition: &'a str,
}

impl <'a> Action<'a> {
    fn remove(&self, token: &mut Token) -> () {
        token.set_value("")
    }

    fn tag(&self, token: &mut Token) -> () {
        token.set_value(match REPLACE_MAPPINGS.get(&self.action_condition as &str) {
            Some(tag) => tag,
            None => token.value,
        })
    }

    fn demojize(&self, token: &mut Token) -> () {
        token.set_value(match emojis::get(&token.value) {
            Some(demoji) => demoji.shortcode().unwrap_or(&token.value),
            _ => token.value,
        })
    }

    fn emojize(&self, token: &mut Token) -> () {
        token.set_value(match emojis::get_by_shortcode(&token.value) {
            Some(_emoji) => _emoji.as_str(),
            _ => token.value,
        })
    }

    fn is_action_valid(&self) -> bool {
        if let Some(action_name) = &self.action_name {
            if action_name.len() == 0 {
                return false
            }
            if let Some(actions) = ACTION_MAPPING.get(&self.action_condition as &str) {
                if !actions.contains(action_name) {
                    panic!(
                        r#"Unknown action {action_name}, expected {expected_actions}"#,
                        action_name=self.action_name.as_deref().unwrap_or_default(), expected_actions=
                        ACTION_MAPPING.get(&self.action_condition as &str).unwrap().join(",")
                    );
                } else {
                    return true
                }
            }
        }
        false
    }

    pub fn apply(&self, token: &mut Token) -> bool {
        if !self.is_action_valid() {
            return false
        }
        let is_condition_matched = match self.action_condition {
            "is_mention" => token.is_mention(),
            "is_hashtag" => token.is_hashtag(),
            "is_url" => token.is_url(),
            "is_digit" => token.is_digit(),
            "is_emoji" => token.is_emoji(),
            "is_emoticon" => token.is_emoticon(),
            "is_punct" => token.is_punct(),
            "is_email" => token.is_email(),
            "is_html_tag" => token.is_html_tag(),
            &_ => false
        };
        if !is_condition_matched {
            return false
        }
        match self.action_name.as_deref() {
            Some("remove") => self.remove(token),
            Some("tag") => self.tag(token),
            Some("demojize") => self.demojize(token),
            Some("emojize") => self.emojize(token),
            _ => return false,
        }
        return true
    }
}
pub struct WeiboToken<'a> {
    token: Token<'a>,
}

impl <'a> WeiboToken<'a> {
    pub fn new(value: &str) -> WeiboToken {
        WeiboToken {token: Token{value: value}}
    }

    pub fn set_value(&mut self, new_value: &'a str) {
        self.token = Token{value: new_value};
    }


    pub fn is_emoji(&self) -> bool {
        self.token.is_emoji()
    }

    pub fn is_punct(&self) -> bool {
        self.token.is_punct()
    }

    pub fn is_hashtag(&self) -> bool {
        self.token.check_flag(*WEIBO_HASHTAG)
    }

    pub fn is_url(&self) -> bool {
        self.token.is_url()
    }

    pub fn is_mention(&self) -> bool {
        self.token.is_mention()
    }

    pub fn is_emoticon(&self) -> bool {
        self.token.is_emoticon()
    }

    pub fn is_digit(&self) -> bool {
        self.token.is_digit()
    }

    pub fn is_email(&self) -> bool {
        self.token.is_email()
    }

    pub fn is_html_tag(&self) -> bool {
        self.token.is_html_tag()
    }

    pub fn do_action(&mut self, action: &Action) -> bool {
        action.apply(&mut self.token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[rstest]
    #[case("#emnlp2019", true)]
    #[case("#prédéfinie", true)]  // non ascii
    #[case("#Филмскисусрети", true)]
    #[case("#정국생일ᄎᄏ", true)]
    #[case("#123", false)]  // # a hashtag can't be just a seq of numbers
    fn test_is_hashtag(#[case] value: &str, #[case] expected: bool) {
        let mut token = Token {value: value};
        assert_eq!(expected, token.is_hashtag())
    }

    #[rstest]
    #[case("https://buff.ly/2Uclr2A", true)]
    #[case("www.google.fr", true)] // # without leading http(s)
    fn test_is_url(#[case] value: &str, #[case] expected: bool) {
        let mut token = Token {value: value};
        assert_eq!(expected, token.is_url())
    }

    #[rstest]
    #[case("@tutu", true)]
    #[case("@@", false)]  // # not valid mention
    #[case("tutu@gmail.com", false)]  // # email
    fn test_is_mention(#[case] value: &str, #[case] expected: bool) {
        let mut token = Token {value: value};
        assert_eq!(expected, token.is_mention())
    }

    #[rstest]
    #[case("😰", true)]
    #[case("joy", true)]  // demojized emoji ('joy' is in the emoji alias)
    #[case("notemoji", false)]
    fn test_is_emoji(#[case] value: &str, #[case] expected: bool) {
        let mut token = Token {value: value};
        assert_eq!(expected, token.is_emoji())
    }

    #[rstest]
    #[case("1", true)] // single number
    #[case("123", true)]  // a sequence of numbers
    #[case("12.34", true)]  // decimal
    #[case("12/34", true)]  // fraction
    #[case("12abc", false)]  // combination of numbers and alphabets
    fn test_is_digit(#[case] value: &str, #[case] expected: bool) {
        let mut token = Token {value: value};
        assert_eq!(expected, token.is_digit())
    }

    #[rstest]
    #[case(",", true)]
    #[case("\u{2019}", true)]
    #[case("12", false)]  // the length of token is not 1
    fn test_is_punct(#[case] value: &str, #[case] expected: bool) {
        let mut token = Token {value: value};
        assert_eq!(expected, token.is_punct())
    }

    #[rstest]
    #[case("tutu@gmail.com", true)]
    #[case("@tutu", false)] // mention
    fn test_is_email(#[case] value: &str, #[case] expected: bool) {
        let mut token = Token {value: value};
        assert_eq!(expected, token.is_email())
    }

    #[test]
    fn test_token_check_flag() {
        let mut token = Token {value: "#hashtag"};
        assert!(token.check_flag(*HASHTAG));
        let mut token = Token {value: "not_hashtag"};
        assert!(!token.check_flag(*HASHTAG))
    }

    #[test]
    fn test_token_do_action_remove() {
        let mut token = Token{value: "#hashtag"};
        token.do_action(&Action{action_name: Some("remove"), action_condition: "is_hashtag"});
        assert_eq!(token.value, "")
    }

    #[test]
    fn test_token_do_action_tag() {
        let mut token = Token{value: "#hashtag"};
        token.do_action(&Action{action_name: Some("tag"), action_condition: "is_hashtag"});
        assert_eq!(token.value, *HASHTAG_TAG)
    }

    #[test]
    fn test_token_do_action_none() {
        let mut token = Token{value: "#hashtag"};
        token.do_action(&Action{action_name: None, action_condition: "is_hashtag"});
        assert_eq!(token.value, "#hashtag")
    }

    #[test]
    fn test_action_remove() {
        // arguments are not important here
        let action = Action{action_name: Some("unittest"), action_condition: "unitest"};
        let mut token = Token{value: "test"};
        action.remove(&mut token);
        assert_eq!(token.value, "")
    }

    #[test]
    fn test_action_demojize() {
        // arguments are not important here
        let action = Action{action_name: Some("unittest"), action_condition: "unitest"};
        let mut token = Token{value: "😀"};
        action.demojize(&mut token);
        assert_eq!(token.value, "grinning")
    }

    #[test]
    fn test_action_emojize() {
        // arguments are not important here
        let action = Action{action_name: Some("unittest"), action_condition: "unitest"};
        let mut token = Token{value: "grinning"};
        action.emojize(&mut token);
        assert_eq!(token.value, "😀")
    }

    #[test]
    fn test_action_tag() {
        let action = Action{action_name: Some("tag"), action_condition: "is_emoji"};
        let mut token = Token{value: "😰"};
        action.tag(&mut token);
        assert_eq!(token.value, "<EMOJI>")
    }

    #[rstest]
    #[case("", "is_hashtag", false)] // action name is empty
    #[case("remove", "is_hashtag", true)]
    #[case("tag", "is_hashtag", true)]
    fn test_action_is_action_valid(#[case] action_name: &str, #[case] action_condition: &str, #[case] expected: bool) {
        let action = Action{action_name: Some(action_name), action_condition: action_condition};
        assert_eq!(action.is_action_valid(), expected)
    }

    #[test]
    fn test_action_is_action_valid_with_none_action_name() {
        let action = Action{action_name: None, action_condition: "is_hashtag"};
        assert_eq!(action.is_action_valid(), false)
    }

    #[test]
    #[should_panic]
    fn test_action_is_action_valid_with_action_is_not_allowed() {
        let action = Action{action_name: Some("emojize"), action_condition: "is_hashtag"};
        assert_eq!(action.is_action_valid(), false)
    }

    #[test]
    fn test_action_apply_returning_true() {
        let action = Action{action_name: Some("remove"), action_condition: "is_hashtag"};
        let mut token = Token{value: "#hashtag"};
        assert_eq!(action.apply(&mut token), true)
    }

    #[test]
    fn test_action_apply_returning_false() {
        let action = Action{action_name: Some("remove"), action_condition: "is_hashtag"};
        let mut token = Token{value: "@hashtag"};
        assert_eq!(action.apply(&mut token), false)
    }

    #[rstest]
    #[case("#中国", false)]
    #[case("#中国#", true)]
    fn test_is_weibo_hashtag(#[case] value: &str, #[case] expected: bool) {
        let mut weibo_token = WeiboToken::new(value);
        assert_eq!(expected, weibo_token.is_hashtag())
    }
}