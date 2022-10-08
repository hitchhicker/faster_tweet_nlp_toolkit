pub mod prep;
pub mod constants;
pub mod utils;

use crate::prep::token::{TokenTrait, Token};
use emojis;

fn main() {
    let mut token = Token {value: String::from("hello")};
    println!("{}", token.value);
    assert!(!token.is_hashtag());
    token.set_value(String::from("#hello"));
    assert!(token.is_hashtag());
    println!("{}", token.value);
    emojis::get("🚀").unwrap();
}
