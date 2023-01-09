use std::borrow::Cow;

use unicode_normalization::UnicodeNormalization;
use unicode_categories::UnicodeCategories;

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

#[cfg(test)]
mod tests {
    use crate::utils::strip_accents_unicode;

    #[test]
    fn test_strip_accents_unicode() {
        assert_eq!(strip_accents_unicode("être").as_ref(), "etre");
    }
}