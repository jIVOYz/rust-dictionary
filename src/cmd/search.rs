use std::process;

use crate::{cmd::{Run, Search}, config};
use dictionary::Dictionary;

impl Run for Search {
    fn run(self: Self) {
        let dictionary = Dictionary::load_from_file(&config::data_file()).expect("failed to load data file");
        let words = dictionary.search_word(&self.query).unwrap_or_else(|| {
            println!("Not found");
            process::exit(0);
        });

        for word in words.iter() {
            print!("{}. {} - ", word.index, word.name);
            let definitions = &word.definition.join(", ");
            println!("{} ", definitions);
        }
    }
}
