use crate::cmd::{List, Run};
use anyhow::Result;
use di::Dictionary;

impl Run for List {
    fn run(self: Self) -> Result<()> {
        let dictionary = Dictionary::load()?;
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
        Ok(())
    }
}
