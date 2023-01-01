#![allow(dead_code)]
#![allow(unused_variables)]
use ftnt::{prep::token::*, constants::{REPLACE_MAPPINGS, ACTION_MAPPING}, regexes::WEIBO_HASHTAG};
use pyo3::prelude::*;

use emojis;

#[pyclass(module = "faster_tweet_nlp_toolkit", name = "Token")]
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct PyToken {
    token: Token,
}
impl From<Token> for PyToken {
    fn from(token: Token) -> Self {
        Self { token }
    }
}

#[pymethods]
impl PyToken {
    #[new]
    pub fn new(value: String) -> PyToken {
        Token::new(value).into()
    }
    // TODO: add setter
    #[pyo3(text_signature = "(self, new_value)")]
    pub fn set_value(&mut self, new_value: String) {
        self.token.set_value(new_value)
    }

    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.token.value.len())
    }

    pub fn is_punct(&self) -> bool {
        self.token.is_punct()
    }

    pub fn is_emoji(&self) -> bool {
        self.token.is_emoji()
    }

    pub fn is_hashtag(&self) -> bool {
        self.token.is_hashtag()
    }
    pub fn is_url(&self) -> bool {
        self.token.is_url()
    }
    pub fn is_mention(&self) -> bool {
        self.token.is_mention()
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

    #[pyo3(text_signature = "(self, action)")]
    pub fn do_action(&mut self, action: &PyAction) -> bool {
        self.token.do_action(&action.action)
    }
}


#[pyclass(module = "faster_tweet_nlp_toolkit", name = "Action")]
pub struct PyAction {
    action: Action,
}

impl From<Action> for PyAction {
    fn from(action: Action) -> Self {
        Self { action }
    }
}

impl From<PyAction> for Action {
    fn from(action: PyAction) -> Self {
        action.action
    }
}

#[pymethods]
impl PyAction {
    #[pyo3(text_signature = "(self, token)")]
    fn remove(&self, token: &mut PyToken) -> () {
        token.set_value("".to_string())
    }

    #[pyo3(text_signature = "(self, token)")]
    fn tag(&self, token: &mut PyToken) -> () {
        token.set_value(match REPLACE_MAPPINGS.get(&self.action.action_condition as &str) {
            Some(tag) => tag.to_string(),
            None => token.token.value.to_string()
        })
    }

    #[pyo3(text_signature = "(self, token)")]
    fn demojize(&self, token: &mut PyToken) -> () {
        token.set_value(match emojis::get(&token.token.value) {
            Some(demoji) => format!(":{}:", demoji.shortcode().unwrap_or(&token.token.value)),
            _ => format!(":{}:", &token.token.value)
        })
    }

    #[pyo3(text_signature = "(self, token)")]
    fn emojize(&self, token: &mut PyToken) -> () {
        token.set_value(match emojis::get_by_shortcode(&token.token.value[1..token.token.value.len()-1]) {
            Some(_emoji) => _emoji.to_string(),
            _ => token.token.value.to_string(),
        })
    }

    fn is_action_valid(&self) -> bool {
        if let Some(action_name) = &self.action.action_name {
            if action_name.len() == 0 {
                return false
            }
            if let Some(actions) = ACTION_MAPPING.get(&self.action.action_condition as &str) {
                if !actions.contains(&&action_name.as_str()) {
                    panic!(
                        r#"Unknown action {action_name}, expected {expected_actions}"#,
                        action_name=self.action.action_name.as_deref().unwrap_or_default(), expected_actions=
                        ACTION_MAPPING.get(&self.action.action_condition as &str).unwrap().join(",")
                    );
                } else {
                    return true
                }
            }
        }
        false
    }

    #[pyo3(text_signature = "(self, token)")]
    pub fn apply(&self, token: &mut PyToken) -> bool {
        if !self.is_action_valid() {
            return false
        }
        let is_condition_matched = match self.action.action_condition.as_str() {
            "is_mention" => token.is_mention(),
            "is_hashtag" => token.is_hashtag(),
            "is_url" => token.is_url(),
            "is_digit" => token.is_digit(),
            "is_emoji" => token.is_emoji(),
            "is_punct" => token.is_punct(),
            "is_email" => token.is_email(),
            "is_html_tag" => token.is_html_tag(),
            &_ => false
        };
        if !is_condition_matched {
            return false
        }
        match self.action.action_name.as_deref() {
            Some("remove") => self.remove(token),
            Some("tag") => self.tag(token),
            Some("demojize") => self.demojize(token),
            Some("emojize") => self.emojize(token),
            _ => return false,
        }
        return true
    }
}

#[pyclass(module = "faster_tweet_nlp_toolkit", name = "WeiboToken")]
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct PyWeiboToken{
    token: WeiboToken,
}

impl From<WeiboToken> for PyWeiboToken {
    fn from(token: WeiboToken) -> Self {
        Self { token}
    }
}

#[pymethods]
impl PyWeiboToken {
    #[new]
    pub fn new(value: String) -> PyWeiboToken {
        WeiboToken::new(value).into()
    }

    #[pyo3(text_signature = "(self, new_value)")]
    pub fn set_value(&mut self, new_value: String) {
        self.token.set_value(new_value)
    }

    pub fn is_emoji(&self) -> bool {
        self.token.is_emoji()
    }

    pub fn is_punct(&self) -> bool {
        self.token.is_punct()
    }

    pub fn is_hashtag(&self) -> bool {
        self.token.token.check_flag(*WEIBO_HASHTAG)
    }

    pub fn is_url(&self) -> bool {
        self.token.is_url()
    }

    pub fn is_mention(&self) -> bool {
        self.token.is_mention()
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

    #[pyo3(text_signature = "(self, action)")]
    pub fn do_action(&mut self, action: &PyAction) -> bool {
        action.action.apply(&mut self.token.token)
    }
}