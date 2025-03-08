use clap::Parser;
use inquire::{
    validator::ValueRequiredValidator,
    Confirm, Text,
};
use std::process;
mod cli;
mod config;
use cli::{AddArgs, Cli, EditArgs};
use dictionary::{Dictionary, Word};

fn main() {
    let cli = Cli::parse();
    let data_file = config::data_file();

    let mut dictionary = Dictionary::load_from_file(&data_file).expect("failed to load file");

    match cli {
        Cli::Add(args) => {
            let AddArgs {
                name,
                definition,
                example,
            } = args;

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
                            example,
                        });
                        println!("Successfully added new word")
                    }
                    Ok(false) => process::exit(0),
                    Err(_) => {
                        eprintln!("Invalid answer");
                        process::exit(1);
                    }
                }
            } else {
                dictionary.add_word(Word {
                    index: dictionary.list.len() + 1,
                    name,
                    definition,
                    example,
                });
                println!("Successfully added new word")
            }
        }
        Cli::Remove(args) => dictionary.remove_word(&args.id).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        }),
        Cli::Edit(args) => {
            let EditArgs { id } = args;

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
        }
        Cli::List(args) => {
            let n = args.last.unwrap_or(dictionary.list.len());

            for word in dictionary.list.iter().rev().take(n).rev() {
                println!(
                    "{}. {} - {}",
                    &word.index,
                    &word.name,
                    &word.definition.join(", ")
                );

                if word.example.is_some() && args.full {
                    println!("Examples: ");
                    let examples = word.example.clone().unwrap().join(", ");
                    println!("{}", examples);
                }
            }
        }
        Cli::Search(args) => {
            let words = dictionary.search_word(&args.query).unwrap_or_else(|| {
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

    dictionary.save();
}
