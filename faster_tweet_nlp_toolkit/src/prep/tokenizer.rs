use crate::prep::token::{Token, WeiboToken};
use crate::prep::regexes::{TWEET_TOKENIZE, WEIBO_TOKENIZE};

pub fn tweet_tokenize(text: String) -> Vec<Token> {
    unsafe {
        TWEET_TOKENIZE.find_iter(&text.as_bytes()).map(
            |m| Token::new(String::from_utf8_unchecked(m.unwrap().subject().to_vec()))).collect()
    }
}

pub fn weibo_tokenize(text: String) -> Vec<WeiboToken> {
    unsafe {
        WEIBO_TOKENIZE.find_iter(text.as_bytes()).map(
            |m| WeiboToken::new(String::from_utf8_unchecked(m.unwrap().subject().to_vec()))).collect()
    }
}

pub fn white_space_tokenize(text: &str) -> Vec<Token> {
    let text = text.trim();
    if text.len() == 0 {
        return vec![]
    }
    return text.split(" ").map(|x| Token{value: x.to_owned()}).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_space_tokenize() {
        let token_values = vec!["@remy:", "This", "is", "waaaaayyyy", "too", "much", "for", "you"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_owned()}).collect();
        itertools::assert_equal(
            white_space_tokenize(" @remy: This is waaaaayyyy too much for you"),
            expected_tokens
        );
    }

    #[test]
    fn test_tweet_tokenize() {
        let token_values = vec!["@remy", ":", "This", "is", "waaaaayyyy", "#too", "much", "for", "you"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_owned()}).collect();
        itertools::assert_equal(
            tweet_tokenize(" @remy: This is waaaaayyyy #too much for you".to_string()),
            expected_tokens
        );
        let token_values = vec!["คลับพาราไดซ์", ",", "จะถูกต้อง", ".", "วันสุดท้ายทุกสิ่งที่ดูเหมือนว่าตกลง"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_owned()}).collect();
        itertools::assert_equal(
            tweet_tokenize(" คลับพาราไดซ์, จะถูกต้อง. วันสุดท้ายทุกสิ่งที่ดูเหมือนว่าตกลง".to_string()),
            expected_tokens
        );

        // TODO: Refact the tokenizer function to make it passed
        // let token_values = vec![":", "http://t.co/skU8zM7Slh", ":joy:"];
        // let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_owned()}).collect();
        // itertools::assert_equal(
        //     tweet_tokenize(":http://t.co/skU8zM7Slh :joy:".to_string()),
        //     expected_tokens
        // );
    }
}