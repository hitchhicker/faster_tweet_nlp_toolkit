#![allow(dead_code)]
#![allow(unused_variables)]

use ftnt::prep::token::*;
use pyo3::prelude::*;
use core::iter::Iterator;

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

impl From<PyToken> for Token {
    fn from(token: PyToken) -> Self {
        token.token
    }
}

#[pyclass]
struct MyIterator {
    iter: Box<dyn Iterator<Item = char> + Send>
}

#[pymethods]
impl MyIterator {
    #[allow(clippy::self_named_constructors)]
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf.into()
    }
    fn __next__(&mut self) -> Option<char> {
        self.iter.next()
    }
}

#[pymethods]
impl PyToken {
    #[new]
    pub fn new(value: String) -> PyToken {
        Token::new(value).into()
    }
    // String like operations
    pub fn __add__(&self, other: String) -> PyResult<String> {
        Ok(concat_string!(self.token.value, other))
    }

    pub fn __radd__(&self, other: String) -> PyResult<String> {
        Ok(concat_string!(other, self.token.value))
    }

    pub fn __mul_func(&self, val: isize) -> PyResult<String> {
        if val <= 0 {
            Ok("".to_string())
        } else {
            Ok(self.token.value.repeat(val.try_into().unwrap()))
        }
    }

    pub fn __iadd__(&mut self, other: String) -> () {
        self.set_value(self.__add__(other).unwrap())
    }

    pub fn __imul__(&mut self, val: isize) -> () {
        self.set_value(self.__mul_func(val).unwrap())
    }

    pub fn __mul__(&self, val: isize) -> PyResult<String> {
        self.__mul_func(val)
    }

    pub fn __rmul__(&self, val: isize) -> PyResult<String> {
        self.__mul_func(val)
    }

    fn __iter__(&self) -> PyResult<MyIterator> {
        let iter = MyIterator {
            iter: Box::new(self.token.value.chars().collect::<Vec<_>>().into_iter()),
        };
        Ok(iter)
    }

    #[getter]
    fn get_value(&self) -> PyResult<String> {
        Ok(self.token.value.clone())
    }

    #[setter]
    pub fn set_value(&mut self, new_value: String) -> () {
        self.token.set_value(new_value);
    }

    fn __str__(&self) -> PyResult<String>   {
        Ok(format!("\"{}\"", self.token.value))
    }

    fn __repr__(&self) -> PyResult<String>   {
        self.__str__()
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.token.value.len())
    }

    fn __getitem__(&mut self, idx: usize) -> PyResult<String> {
        Ok(self.token.index_at(idx).to_string())
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
        self.action.tag(&mut token.token)
    }

    #[pyo3(text_signature = "(self, token)")]
    fn demojize(&self, token: &mut PyToken) -> () {
        self.action.demojize(&mut token.token)
    }

    #[pyo3(text_signature = "(self, token)")]
    fn emojize(&self, token: &mut PyToken) -> () {
        self.action.emojize(&mut token.token)
    }

    fn is_action_valid(&self) -> bool {
        self.action.is_action_valid()
    }

    #[pyo3(text_signature = "(self, token)")]
    pub fn apply(&self, token: &mut PyToken) -> bool {
        self.action.apply(&mut token.token)
    }
}
