use std::fs::File;
use vibrato::Dictionary;

use super::constants::DICTIONARY_PATH;

pub fn load_dictionary() -> Dictionary {
    let reader = zstd::Decoder::new(File::open(DICTIONARY_PATH).unwrap()).unwrap();
    Dictionary::read(reader).unwrap()
}
