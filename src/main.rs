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
                word,
                meaning,
                example,
            } = args;

            dictionary.add_word(Word {
                index: dictionary.list.len() + 1,
                word,
                meaning,
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
                print!("{}. {} - ", &word.index, &word.word);

                let m = &word.meaning.join(", ");
                println!("{} ", m);

                if word.example.is_some() && args.full {
                    println!("Examples: ");
                    let examples = word.example.clone().unwrap().join(", ");
                    println!("{}", examples);
                }
            }
        }
    }

    dictionary.save();
}
