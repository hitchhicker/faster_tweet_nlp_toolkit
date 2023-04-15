use std::{collections::HashSet, fs::File};
use std::io::{BufRead, BufReader, LineWriter, Write};
use super::{text_parser::parse_text, token::Token};

pub fn prep(
    text: String,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>,
    tokenizer: Option<fn(String) -> Vec<Token>>,
    filters: Option<HashSet<&str>>,
    emojis: Option<&str>,
    emoticons: Option<&str>,
    mentions: Option<&str>,
    hashtags: Option<&str>,
    urls: Option<&str>,
    digits: Option<&str>,
    puncts: Option<&str>,
    emails: Option<&str>,
    html_tags: Option<&str>,
) -> String {
    return parse_text(
        text,
        encoding,
        remove_unencodable_char,
        to_lower,
        strip_accents,
        reduce_len,
        tokenizer,
        filters,
        emojis,
        emoticons,
        mentions,
        hashtags,
        urls,
        digits,
        puncts,
        emails,
        html_tags,
    ).value().to_string()
}

pub fn prep_file(
    filename: &str,
    outfile: &str,
    encoding: Option<&str>,
    remove_unencodable_char: Option<bool>,
    to_lower: Option<bool>,
    strip_accents: Option<bool>,
    reduce_len: Option<bool>,
    tokenizer: Option<fn(String) -> Vec<Token>>,
    filters: Option<HashSet<&str>>,
    emojis: Option<&str>,
    emoticons: Option<&str>,
    mentions: Option<&str>,
    hashtags: Option<&str>,
    urls: Option<&str>,
    digits: Option<&str>,
    puncts: Option<&str>,
    emails: Option<&str>,
    html_tags: Option<&str>,
) {
    let reader: BufReader<File> = match File::open(filename) {
        Err(why) => panic!("Couldn't open {}: {}", filename, why),
        Ok(file) => BufReader::new(file),
    };
    let mut writer: LineWriter<File> = match File::create(outfile) {
        Err(why) => panic!("Couldn't open {}: {}", outfile, why),
        Ok(file) =>  LineWriter::new(file),
    };
    for line in reader.lines() {
        let preprocessed_text = prep(
            line.unwrap(),
            encoding,
            remove_unencodable_char,
            to_lower,
            strip_accents,
            reduce_len,
            tokenizer,
            filters.clone(),
            emojis,
            emoticons,
            mentions,
            hashtags,
            urls,
            digits,
            puncts,
            emails,
            html_tags,
        );
        writer.write_all(preprocessed_text.as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}