use crate::prep::token::{Token, WeiboToken};
use crate::prep::regexes::{TWEET_TOKENIZE, WEIBO_TOKENIZE};

pub fn tweet_tokenize(text: &str) -> Vec<Token> {
    TWEET_TOKENIZE.find_iter(&text).map(
        |m| Token{value: m.as_str().to_string()}).collect()
}

pub fn _weibo_tokenize(text: &str) -> Vec<WeiboToken> {
    WEIBO_TOKENIZE.find_iter(&text).map(
        |m| WeiboToken::new(m.as_str().to_string())).collect()
}

pub fn white_space_tokenize(text: &str) -> Vec<Token> {
    let text = text.trim();
    if text.len() == 0 {
        return vec![]
    }
    return text.split(" ").map(|x| Token{value: x.to_string()}).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_space_tokenize() {
        let token_values = vec!["@remy:", "This", "is", "waaaaayyyy", "too", "much", "for", "you"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_string()}).collect();
        itertools::assert_equal(
            white_space_tokenize(" @remy: This is waaaaayyyy too much for you"),
            expected_tokens
        );
    }

    #[test]
    fn test_tweet_tokenize() {
        let token_values = vec!["@remy", ":", "This", "is", "waaaaayyyy", "too", "much", "for", "you"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_string()}).collect();
        itertools::assert_equal(
            tweet_tokenize(" @remy: This is waaaaayyyy too much for you"),
            expected_tokens
        );
        let token_values = vec!["คลับพาราไดซ์", ",", "จะถูกต้อง", ".", "วันสุดท้ายทุกสิ่งที่ดูเหมือนว่าตกลง"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_string()}).collect();
        itertools::assert_equal(
            tweet_tokenize(" คลับพาราไดซ์, จะถูกต้อง. วันสุดท้ายทุกสิ่งที่ดูเหมือนว่าตกลง"),
            expected_tokens
        );
    }
}