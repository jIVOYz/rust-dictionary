use crate::cmd::{List, Run};
use crate::config;
use di::Dictionary;

impl Run for List {
    fn run(self: Self) {
        let dictionary = Dictionary::load_from_file(&config::data_file()).expect("failed to read data file");
        let n = self.last.unwrap_or(dictionary.list.len());

        for word in dictionary.list.iter().rev().take(n).rev() {
            println!(
                "{}. {} - {}",
                &word.index,
                &word.name,
                &word.definition.join(", ")
            );

            if word.example.is_some() && self.full {
                println!("Examples: ");
                let examples = word.example.clone().unwrap().join(", ");
                println!("{}", examples);
            }
        }
    }
}
