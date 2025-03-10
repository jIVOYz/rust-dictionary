use di::Dictionary;
use std::process;

use crate::{
    cmd::{Remove, Run},
    config,
};

impl Run for Remove {
    fn run(self: Self) {
        let mut dictionary =
            Dictionary::load_from_file(&config::data_file()).expect("failed to read data file");

        dictionary.remove_word(&self.id).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        dictionary.save();
    }
}
