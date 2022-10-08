#![allow(dead_code, unused)]
use regex::Regex;
use crate::prep::regexes::*;
use crate::constants::*;
use unicode_categories::UnicodeCategories;
use emojis;


#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Token {
    pub value: String,
}

impl Token {
    pub fn set_value(&mut self, new_value: String) {
        self.value = new_value;
    }

    // TODO: __getitem__ and __setitem__

    pub fn _check_flag(&self, pattern: &str) -> bool {
        let compiled_pattern:Regex = Regex::new(&format!(r"^{}$", pattern)).unwrap();
        compiled_pattern.is_match(&self.value)
    }

    pub fn is_hashtag(&self) -> bool {
        !self._check_flag(*NOT_A_HASHTAG) & self._check_flag(*HASHTAG)
    }

    pub fn is_url(&self) -> bool {
        self._check_flag(*URL)
    }

    pub fn is_mention(&self) -> bool {
        self._check_flag(*MENTION)
    }

    pub fn is_emoticon(&self) -> bool {
        self._check_flag(*EMOTICONS)
    }

    pub fn is_emoji(&self) -> bool {
        let emoji_opt = emojis::get(&self.value);
        match emoji_opt {
            Some(_emoji) => true,
            _ => false
        }
    }

    pub fn is_digit(&self) -> bool {
        self._check_flag(*DIGIT)
    }

    pub fn is_email(&self) -> bool {
        self._check_flag(*EMAIL)
    }

    pub fn is_html_tag(&self) -> bool {
        self._check_flag(*HTML_TAG)
    }

    pub fn is_punct(&self) -> bool {
        self.value.len() == 1 && self.value.chars().next().unwrap().is_punctuation()
    }
}

pub struct Action {
    action_name: String,
    action_condition: String,
}

impl Action {
    fn remove(&self, token: &mut Token) -> () {
        token.set_value("".to_string())
    }

    fn tag(&self, token: &mut Token) -> () {
        token.set_value(match REPLACE_MAPPINGS.get(&self.action_condition as &str) {
            Some(tag) => tag.to_string(),
            None => token.value.to_string(),
        })
    }

    fn demojize(&self, token: &mut Token) -> () {
        token.set_value(match emojis::get(&token.value) {
            Some(demoji) => demoji.name().to_string(),
            _ => token.value.to_string(),
        })
    }

    fn emojize(&self, token: &mut Token) -> () {
        token.set_value(match emojis::get_by_shortcode(&token.value) {
            Some(_emoji) => _emoji.name().to_string(),
            _ => token.value.to_string(),
        })
    }

    fn check_action(&self) -> () {
        match ACTION_MAPPING.get(&self.action_condition as &str)  {
            Some(actions) =>  {
                if !actions.contains(&self.action_name.as_str()) {
                    panic!(
                        "Unknown action {action_name}, expected {expected_actions}",
                        action_name=self.action_name, expected_actions=
                        ACTION_MAPPING.get(&self.action_condition as &str).unwrap().join(",")
                    );
                }
            }
            _ => return,
        }
    }

    pub fn apply(&self, token: &mut Token) -> bool {
        self.check_action();
        let is_condition_matched = match self.action_condition.as_str() {
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
        if is_condition_matched {
            match self.action_name.as_str() {
                "remove" => self.remove(token),
                "tag" => self.tag(token),
                "demojize" => self.demojize(token),
                "emojize" => self.emojize(token),
                &_ => return false,
            }
        }
        return false
    }
}