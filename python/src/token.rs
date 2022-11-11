#![allow(dead_code)]
#![allow(unused_variables)]
use ftnt::prep::token::*;
use pyo3::prelude::*;

#[pyclass(module = "faster_tweet_nlp_toolkit", name = "Token")]
#[derive(Clone)]
pub struct PyToken {
    token: Token,
}