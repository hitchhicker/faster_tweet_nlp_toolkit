use crate::prep::token::Token;
use crate::prep::regexes::TWEET_TOKENIZE;

pub fn tweet_tokenize(text: String) -> Vec<Token> {
    unsafe {
        TWEET_TOKENIZE.find_iter(&text.as_bytes()).map(
            |m| Token::new(String::from_utf8_unchecked(m.unwrap().subject().to_vec()))).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let token_values = vec!["http://t.co/skU8zM7Slh", ":joy:"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_owned()}).collect();
        itertools::assert_equal(
            tweet_tokenize("http://t.co/skU8zM7Slh :joy:".to_string()),
            expected_tokens
        );
        let token_values = vec!["123", "@hello", "#world", r"\(^o^)/", "www.url.com", ":)", "abc@gmail.com"];
        let expected_tokens: Vec<Token> = token_values.into_iter().map(|x| Token{value: x.to_owned()}).collect();
        itertools::assert_equal(
            tweet_tokenize(r"123 @hello #world \(^o^)/ www.url.com :) abc@gmail.com".to_string()),
            expected_tokens
        );
    }
}