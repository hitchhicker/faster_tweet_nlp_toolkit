#![allow(dead_code, unused)]
use std::collections::HashSet;

static EMOJI_TAG: &'static str = "<EMOJI>";
static MENTION_TAG: &'static str = "<MENTION>";
static HASHTAG_TAG: &'static str = "<HASHTAG>";
static UNENCODABLE_CHAR: &'static str = "�";
static URL_TAG: &'static str = "<URL>";
static DIGIT_TAG: &'static str = "<DIGIT>";
static EMOTICON_TAG: &'static str = "<EMOTICON>";
static PUNCTUATION_TAG: &'static str = "<PUNCT>";
static EMAIL_TAG: &'static str = "<EMAIL>";
static  VARIATION_SELECTORS: [&'static str; 16] = [
    "\u{fe00}",
    "\u{fe01}",
    "\u{fe02}",
    "\u{fe03}",
    "\u{fe04}",
    "\u{fe05}",
    "\u{fe06}",
    "\u{fe07}",
    "\u{fe08}",
    "\u{fe09}",
    "\u{fe0a}",
    "\u{fe0b}",
    "\u{fe0c}",
    "\u{fe0d}",
    "\u{fe0e}",
    "\u{fe0f}",
];
static JAPANESE_LANGUAGE_CODE: &'static str = "ja";
static CHINESE_LANGUAGE_CODE: &'static str = "zh";
static THAI_LANGUAGE_CODE: &'static str = "th";
static SUPPORTED_LANGUAGES: [&'static str; 3] = [
    JAPANESE_LANGUAGE_CODE,
    CHINESE_LANGUAGE_CODE,
    THAI_LANGUAGE_CODE,
];