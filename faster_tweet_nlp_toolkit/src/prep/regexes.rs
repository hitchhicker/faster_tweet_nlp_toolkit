/*
Important note:
The following expressions are modified from
    https://github.com/cbaziotis/ekphrasis/blob/master/ekphrasis/regexepub(crate) pub(crate) s/generate_expressions.py
and
    https://www.nltk.org/_modules/nltk/tokenize/casual.html#TweetTokenizer
*/
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use pcre2::bytes::{RegexBuilder, Regex};
use std::sync::Mutex;


fn string_to_static_str(s: String) -> &'static str {
    // https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
    Box::leak(s.into_boxed_str())
}

static REGEX_BUILDER: Lazy<Mutex<RegexBuilder>> = Lazy::new(|| {
    let mut builder: RegexBuilder = RegexBuilder::new();
    builder.ucp(true);
    Mutex::new(builder)
});


lazy_static! {
    pub static ref HASHTAG: &'static str = r#"\#\b[\w\-_]+\b"#;
    pub static ref HASHTAG_RE: Regex = REGEX_BUILDER.lock().unwrap().build(r#"\#\b[\w\-_]+\b"#).unwrap();

    pub static ref NOT_A_HASHTAG: &'static str = r#"\#\b[p{N}]+\b"#;
    pub static ref NOT_A_HASHTAG_RE: Regex = REGEX_BUILDER.lock().unwrap().build(r#"^\#\b[\p{N}]+\b$"#).unwrap();

    pub static ref WORD: &'static str = r#"(?:[^\W\d|(?:_](?:[^\W\d_]|['\-_]|[\x{0E00}-\x{0E7F}])+[^\W\d_]?)[^\W\d]?"#;

    pub static ref MENTION:&'static str = r#"@\w+"#;
    pub static ref MENTION_RE: Regex = REGEX_BUILDER.lock().unwrap().build(r#"^@\w+$"#).unwrap();
    pub static ref _LTR_EMOTICON: [&'static str; 5] = [
        // optional hat
        r#"(?:(?<![a-zA-Z])[DPO]|(?<!\d)[03]|[|}><=])?"#,
        // eyes
        r#"(?:(?<![a-zA-Z\(])[xXB](?![a-ce-oq-zA-CE-OQ-Z,\.\/])|(?<![:])[:=|](?![\.])|(?<![%#\d])[%#](?![%#\d])|(?<![\d\$])[$](?![\d\.,\$])|[;](?!\()|(?<![\d\(\-\+])8(?![\da-ce-zA-CE-Z\\/])|\*(?![\*\d,.]))"#,
        // pylint: disable=line-too-long
        // optional tears
        r#"(?:['\",])?"#,
        // optional nose
        r#"(?:(?<![\w*])[oc](?![a-zA-Z])|(?:[-‑^]))?"#,
        // mouth
        r#"(?:[(){}\[\]<>|/\\]+|[Þ×þ]|(?<!\d)[30](?!\d)|(?<![\d\*])[*,.@#&](?![\*\d,.])|(?<![\d\$])[$](?![\d\.,\$])|[DOosSJLxXpPbc](?![a-zA-Z]))"#,
    ];
    pub static ref _RTL_EMOTICON: [&'static str; 7] = [
        r#"(?<![\w])"#,
        r#"(?:[(){}\[\]<>|/\\]+|(?<![\d\.\,])[0](?![\d\.])|(?![\d\*,.@#&])[*,.@#&]|[$]|(?<![a-zA-Z])[DOosSxX])"#,
        // mouth
        r#"(?:[-‑^])?"#,  // optional nose
        r#"(?:['\",])?"#,  // optional tears
        r#"(?:[xX]|[:=|]|[%#]|[$8](?![\d\.])|[;]|\*)"#,  // eyes
        r#"(?:[O]|[0]|[|{><=])?"#,  // optional hat
        r#"(?![a-zA-Z])"#,
    ];
    static ref _LTR_FACE: &'static str = string_to_static_str(_LTR_EMOTICON.map(|x| x.to_string()).join(""));
    static ref _RTL_FACE: &'static str = string_to_static_str(_RTL_EMOTICON.map(|x| x.to_string()).join(""));
    static ref _EASTERN_EMOTICONS: &'static str = r#"(?<![\w])(?:(?:[<>]?[\^;][\W_m][\;^][;<>]?)|(?:[^\s()]?m?[\(][\W_oTOJ]{1,3}[\s]?[\W_oTOJ]{1,3}[)]m?[^\s()]?)|(?:\*?[v>\-\/\\][o0O\_\.][v\-<\/\\]\*?)|(?:[oO0>][\-_\/oO\.\\]{1,2}[oO0>])|(?:\^\^))(?![\w])"#;
    static ref _REST_EMOTICONS: &'static str = r#"(?<![A-Za-z0-9/()])(?:(?:\^5)|(?:\<3))(?![[A-Za-z0-9/()])"#;
    static ref _EMOTICONS: [&'static str; 4] = [
        &_LTR_FACE,
        &_RTL_FACE,
        &_EASTERN_EMOTICONS,
        &_REST_EMOTICONS,
    ];
    pub static ref EMOTICONS: String = _EMOTICONS.map(|x| x.to_string()).join("|");

    pub static ref EMOTICONS_RE: Regex = REGEX_BUILDER.lock().unwrap().build(format!(r#"^{}$"#, *EMOTICONS).as_str()).unwrap();

    pub static ref EMAIL: &'static str = r#"(?:^|)(?:[\w+-](?:\.)?)*?[\w+-]@(?:\w-?)*?\w+(?:\.(?:[a-z]{2,})){1,3}(?:$|)"#;
    pub static ref EMAIL_RE: Regex = REGEX_BUILDER.lock().unwrap().build(r#"^(?:^|)(?:[\w+-](?:\.)?)*?[\w+-]@(?:\w-?)*?\w+(?:\.(?:[a-z]{2,})){1,3}(?:$|)$"#).unwrap();

    pub static ref URL: &'static str = r#"(?:https?://[^\s\.]+\.[^\s]{2,}|www\.[^\s]+\.[^\s]{2,})"#;
    pub static ref URL_RE: Regex = REGEX_BUILDER.lock().unwrap().build(r#"^(?:https?://[^\s\.]+\.[^\s]{2,}|www\.[^\s]+\.[^\s]{2,})$"#).unwrap();

    pub static ref CAMEL_SPLIT: &'static str = r#"((?<=[a-z])[A-Z]|(?<!^)[A-Z](?=[a-z])|[0-9]+|(?<=[0-9\\-\\_])[A-Za-z]|[\\-\\_])"#;

    pub static ref HTML_TAG: &'static str = r#"<[^>\s]+>"#;
    pub static ref HTML_TAG_RE: Regex = REGEX_BUILDER.lock().unwrap().build(r#"^<[^>\s]+>$"#).unwrap();

    pub static ref ASCII_ARROW: &'static str = r#"[\-]+>|<[\-]+"#;

    pub static ref DIGIT: &'static str = r#"(?:[+\-]?\d+[,/.:-]?\d*[+\-]?)"#;
    pub static ref DIGIT_RE: Regex = REGEX_BUILDER.lock().unwrap().build(r#"^(?:[+\-]?\d+[,/.:-]?\d*[+\-]?)$"#).unwrap();

    pub static ref ELLIPSIS_DOTS: &'static str = r#"(?:\.(?:\s*\.){1,})"#;
    pub static ref EMOJI_STRING: &'static str = r#"(?::\w+:)"#;

    // === Patterns ===
    pub static ref QUOTES_PAT: Regex = REGEX_BUILDER.lock().unwrap().build(r#"[“”«»]"#).unwrap();
    pub static ref APOSTROPHES_PAT: Regex = REGEX_BUILDER.lock().unwrap().build(r#"[‘’]"#).unwrap();
    pub static ref URL_PAT: Regex = REGEX_BUILDER.lock().unwrap().build(&URL).unwrap();
    pub static ref RT_MENTION_PAT: Regex = REGEX_BUILDER.lock().unwrap().build(&(r#"^RT "#.to_string() + &MENTION + &r#": "#.to_string())).unwrap();

    // join all together
    static ref _TOKEN_PIPELINE: [&'static str; 11] = [
        &URL, &EMAIL, &MENTION, &HASHTAG, &HTML_TAG, &ASCII_ARROW, &DIGIT, &ELLIPSIS_DOTS, &EMOJI_STRING, &WORD, r#"\S"#
    ];
    static ref TOKEN_PIPELINE: &'static str = string_to_static_str(_TOKEN_PIPELINE.map(|x| x.to_string()).join(r"|"));
    pub static ref TWEET_TOKENIZE: Regex = REGEX_BUILDER.lock().unwrap().build(&TOKEN_PIPELINE).unwrap();
}

