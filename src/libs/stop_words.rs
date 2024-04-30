use std::fs;

use super::constants::CUSTOM_STOP_WORDS_PATH;

pub fn create_stop_words() -> Vec<String> {
    let general_stop_words = stop_words::get(stop_words::LANGUAGE::English);

    let contents = fs::read_to_string(CUSTOM_STOP_WORDS_PATH).unwrap();
    let custom_stop_words: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    [general_stop_words, custom_stop_words].concat()
}
