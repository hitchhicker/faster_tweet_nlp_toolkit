use unicode_normalization::UnicodeNormalization;
use unicode_categories::UnicodeCategories;

use crate::constants::VARIATION_SELECTORS;

pub fn strip_accents_unicode(text: &str) -> String {
    let normlized_text = UnicodeNormalization::nfd(text).collect::<String>();
    let mut output: String = String::new();
    for ch in normlized_text.chars() {
        if !ch.is_mark_nonspacing() {
            output.push(ch);
        }
    }
    return output
}

/*
def remove_variation_selectors(text):
    """Remove styling glyph variants for Unicode characters.
    For instance, remove skin color from emojis.
    """
    for var in VARIATION_SELECTORS:
        text = text.replace(var, "")
    return text
*/

pub fn remove_variation_selectors(text: &str) -> String {
    let mut t = String::from(text);
    for var in VARIATION_SELECTORS.iter().collect::<Vec<_>>(){
        t = t.replace(var, "");
    }
    return t
}