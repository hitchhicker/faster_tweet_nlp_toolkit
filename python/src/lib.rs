extern crate faster_tweet_nlp_toolkit as ftnt;
use pyo3::prelude::*;
mod token;
mod text_parser;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "faster_tweet_nlp_toolkit")]
fn faster_tweet_nlp_toolkit(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<token::PyToken>()?;
    m.add_class::<text_parser::PyParsedText>()?;
    Ok(())
}
