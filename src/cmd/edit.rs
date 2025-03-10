use std::process;

use crate::cmd::{Edit, Run};
use crate::config;
use di::Dictionary;
use inquire::{validator::ValueRequiredValidator, Text};

impl Run for Edit {
    fn run(self: Self) {
        let mut dictionary =
            Dictionary::load_from_file(&config::data_file()).expect("failed to load data file");
        let Edit { id } = self;

        let word = dictionary.list.iter_mut().find(|word| word.index == id);

        if let Some(word) = word {
            let new_name = Text::new("Name: ")
                .with_initial_value(&word.name)
                .with_validator(ValueRequiredValidator::default())
                .prompt();

            if let Ok(name) = new_name {
                word.name = name.trim().to_string();
            }

            println!("Current definitions: {}", word.definition.join(", "));
            let new_definitions = Text::new("Definition(s): ")
                .with_help_message("Multiple definitions should be seperated by comma")
                .with_initial_value(&word.definition.join(", "))
                .with_validator(ValueRequiredValidator::default())
                .prompt();

            if let Ok(definitions) = new_definitions {
                word.definition = definitions
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        } else {
            eprintln!("Word not found");
            process::exit(1);
        }
        dictionary.save();
    }
}
