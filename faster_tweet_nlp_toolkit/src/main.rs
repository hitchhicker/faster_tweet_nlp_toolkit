pub mod prep;
pub mod constants;
pub mod utils;

use crate::prep::token::{TokenTrait, Token};

fn main() {
    let token = Token {value: String::from("www.google.com")};
    println!("{}", token.value);
    assert!(token.is_url());
}
