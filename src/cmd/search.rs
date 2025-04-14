use crate::{
    cmd::{Run, Search},
    config,
};
use anyhow::{bail, Result};
use di::Dictionary;

impl Run for Search {
    fn run(self: Self) -> Result<()> {
        let dictionary =
            Dictionary::load_from_file(&config::data_file()).expect("failed to load data file");
        let words = match dictionary.search_word(&self.query) {
            Some(val) => val,
            None => bail!("word not found"),
        };

        for word in words.iter() {
            let definitions = &word.definition.join(", ");
            println!("{}. {} - {}", word.index, word.name, definitions);
        }
        Ok(())
    }
}
