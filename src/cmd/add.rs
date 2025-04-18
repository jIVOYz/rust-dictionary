use anyhow::{bail, Result};
use inquire::Confirm;

use crate::cmd::{Add, Run};
use crate::config;
use di::{Dictionary, Word};

impl Run for Add {
    fn run(self: Self) -> Result<()> {
        let mut dictionary =
            Dictionary::load_from_file(&config::data_file()).expect("failed to load data file");
        let Add {
            name,
            definition,
            example,
        } = self;

        let find_word = &dictionary.list.iter().find(|word| word.name == name);

        if let Some(_) = find_word {
            let answer = Confirm::new("This word already exists. Do you want to continue?")
                .with_default(false)
                .prompt();

            match answer {
                Ok(true) => {
                    dictionary.add_word(Word {
                        index: dictionary.list.len() + 1,
                        name: name.trim().to_string(),
                        definition: definition.iter().map(|d| d.trim().to_string()).collect(),
                        example: example.clone(),
                    });
                }
                Ok(false) => return Ok(()),
                Err(_) => bail!("invalid answer"),
            }
        } else {
            dictionary.add_word(Word {
                index: dictionary.list.len() + 1,
                name,
                definition,
                example,
            });
        }
        dictionary.save()?;
        Ok(())
    }
}
