pub mod prep;
pub mod constants;
pub mod utils;

use regex::Regex;

fn main() {
    // let emojit_regex: Regex = Regex::new(r#"(?::\w+:)"#).unwrap();
    // for mat in emojit_regex.find_iter(":http:") {
    //     println!("{:?}", mat);
    // }
    let pattern:Regex = Regex::new(r#"([^ ])(https?://)"#).unwrap();
    let text = "asylum seeker:http://t.co/skU8zM7Slh";
    for mat in pattern.find_iter(text) {
        println!("{:?}", mat);
    }
    let text2 = String::from(pattern.replace_all(&text, "$1 $2"));
    println!("{}", text2)
}
