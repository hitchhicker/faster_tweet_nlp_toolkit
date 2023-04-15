extern crate faster_tweet_nlp_toolkit as ftnt;
use pyo3::prelude::*;
mod token;
mod text_parser;
mod text_prep;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "faster_tweet_nlp_toolkit")]
fn faster_tweet_nlp_toolkit(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<token::PyToken>()?;
    m.add_class::<text_parser::PyParsedText>()?;
    m.add_function(wrap_pyfunction!(text_parser::parse_text, m)?)?;
    m.add_function(wrap_pyfunction!(text_parser::preprocess_text, m)?)?;
    m.add_function(wrap_pyfunction!(text_prep::prep, m)?)?;
    m.add_function(wrap_pyfunction!(text_prep::prep_file, m)?)?;
    Ok(())
}
