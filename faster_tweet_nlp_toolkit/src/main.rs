pub mod prep;
pub mod constants;
pub mod utils;

use crate::prep::token::Token;

fn main() {
    let token = Token {value: "www.google.com".to_owned()};
    println!("{}", token.value);
    assert!(token.is_url());
}
