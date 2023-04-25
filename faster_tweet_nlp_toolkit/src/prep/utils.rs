use std::borrow::Cow;

use pcre2::bytes::Regex;
use unicode_normalization::UnicodeNormalization;
use unicode_categories::UnicodeCategories;
use lazy_static::lazy_static;
use crate::constants::VARIATION_SELECTORS;

pub fn strip_accents_unicode(text: &str) -> Cow<String> {
    let normlized_text = UnicodeNormalization::nfd(text).collect::<String>();
    let mut output: String = String::with_capacity(text.len());
    for ch in normlized_text.chars(){
        if !ch.is_mark_nonspacing() {
            output.push(ch);
        }
    }
    return Cow::Owned(output)
}

pub fn remove_variation_selectors(text: &str) -> String {
    let mut t = String::from(text);
    for var in VARIATION_SELECTORS.iter().collect::<Vec<_>>(){
        t = t.replace(var, "");
    }
    return t
}

pub fn preprocess_url(text: &str) -> String {
    lazy_static! {
        static ref HTTP_RE: Regex = Regex::new(r#"([^ ])(https?://)"#).unwrap();
    }
    let pattern: &Regex = &HTTP_RE;
    let t = String::from_utf8(pattern.replace_all(text.as_bytes(), "$1 $2".as_bytes()).to_vec()).unwrap();
    return t
}

#[cfg(test)]
mod tests {
    use crate::prep::utils::*;

    #[test]
    fn test_strip_accents_unicode() {
        assert_eq!(strip_accents_unicode("être").as_ref(), "etre");
    }

    #[test]
    fn test_preprocess_url() {
        assert_eq!(preprocess_url(":http://t.co/skU8zM7Slh"), ": http://t.co/skU8zM7Slh");
    }
}