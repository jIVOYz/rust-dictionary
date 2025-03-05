use clap::Parser;
use std::process;
mod cli;
mod config;
use cli::{AddArgs, Cli};
use dictionary::{Dictionary, Word};

fn main() {
    let cli = Cli::parse();
    let data_file = config::data_file();

    let mut dictionary = Dictionary::load_from_file(&data_file).expect("failed to load file");

    match cli {
        Cli::Add(args) => {
            let AddArgs {
                name: word,
                definition,
                example,
            } = args;

            dictionary.add_word(Word {
                index: dictionary.list.len() + 1,
                name: word,
                definition,
                example,
            });
            println!("Successfully added new word")
        }
        Cli::Remove(args) => dictionary.remove_word(&args.id).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        }),
        Cli::List(args) => {
            let n = args.last.unwrap_or(dictionary.list.len());

            for word in dictionary.list.iter().rev().take(n).rev() {
                print!("{}. {} - ", &word.index, &word.name);

                let m = &word.definition.join(", ");
                println!("{} ", m);

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
