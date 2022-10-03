/*
Important note:
The following expressions are modified from
    https://github.com/cbaziotis/ekphrasis/blob/master/ekphrasis/regexes/generate_expressions.py
and
    https://www.nltk.org/_modules/nltk/tokenize/casual.html#TweetTokenizer
*/
use lazy_static::lazy_static;
use regex::Regex;


fn string_to_static_str(s: String) -> &'static str {
    // https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
    Box::leak(s.into_boxed_str())
}

lazy_static! {
    static ref HASHTAG: &'static str = r#"\#\b[\w\-\_]+\b"#;
    static ref WEIBO_HASHTAG: &'static str = r#"\#[^#]+#"#;
    static ref NOT_A_HASHTAG: &'static str = r#"\#\b[\d]+\b"#;
    static ref WORD: &'static str = r#"(?:[^\W\d|(?:_](?:[^\W\d_]|['\-_]|[\u0e00-\u0e7f])+[^\W\d_]?)[^\W\d\w_]?"#;
    static ref MENTION:&'static str = r#"\@\w+"#;
    static ref _LTR_EMOTICON: [&'static str; 5] = [
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
    static ref _RTL_EMOTICON: [&'static str; 7] = [
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
    static ref EMOTICONS: &'static str = string_to_static_str(_EMOTICONS.map(|x| x.to_string()).join(""));
    static ref EMAIL: &'static str = r#"(?:^|(?<=[^\w@.)]))(?:[\w+-](?:\.(?!\.))?)*?[\w+-]@(?:\w-?)*?\w+(?:\.(?:[a-z]{2,})){1,3}(?:$|(?=\b))"#;
    static ref URL: &'static str = r#"(?:https?:\/\/(?:www\.|(?!www))[^\s\.]+\.[^\s]{2,}|www\.[^\s]+\.[^\s]{2,})"#;
    static ref CAMEL_SPLIT: &'static str = r#"((?<=[a-z])[A-Z]|(?<!^)[A-Z](?=[a-z])|[0-9]+|(?<=[0-9\\-\\_])[A-Za-z]|[\\-\\_])"#;
    static ref HTML_TAG: &'static str = r#"<[^>\s]+>"#;
    static ref ASCII_ARROW: &'static str = r#"[\-]+>|<[\-]+"#;
    static ref DIGIT: &'static str = r#"(?:[+\-]?\d+[,/.:-]?\d*[+\-]?)"#;
    static ref ELLIPSIS_DOTS: &'static str = r#"(?:\.(?:\s*\.){1,})"#;
    static ref EMOJI_STRING: &'static str = r#"(?::\w+:)"#;

    // === Patterns ===
    static ref QUOTES_PAT: Regex = Regex::new(r#"[“”«»]"#).unwrap();
    static ref APOSTROPHES_PAT: Regex = Regex::new(r#"[‘’]"#).unwrap();
    static ref URL_PAT: Regex = Regex::new(&URL).unwrap();
    static ref RT_MENTION_PAT: Regex = Regex::new(&(r#"^RT "#.to_string() + &MENTION + &r#": "#.to_string())).unwrap();

    // join all together
    static ref _TOKEN_PIPELINE: [&'static str; 12] = [
        &URL, &EMAIL, &MENTION, &HASHTAG, &EMOTICONS, &HTML_TAG, &ASCII_ARROW, &DIGIT, &ELLIPSIS_DOTS, &EMOJI_STRING, &WORD, r#"\S"#
    ];
    static ref TOKEN_PIPELINE: &'static str = string_to_static_str(_TOKEN_PIPELINE.map(|x| x.to_string()).join(r"|"));
    static ref TWEET_TOKENIZE: Regex = Regex::new(&TOKEN_PIPELINE).unwrap();

    static ref _WEIBO_TOKEN_PIPELINE: [&'static str; 12] = [
        &URL, &EMAIL, &MENTION, &HASHTAG, &EMOTICONS, &HTML_TAG, &ASCII_ARROW, &DIGIT, &ELLIPSIS_DOTS, &EMOJI_STRING, &WORD, r#"\S"#
    ];
    static ref WEIBO_TOKEN_PIPELINE: &'static str = string_to_static_str(_TOKEN_PIPELINE.map(|x| x.to_string()).join(r"|"));
    static ref WEIBO_TOKENIZE: Regex = Regex::new(&TOKEN_PIPELINE).unwrap();
}

