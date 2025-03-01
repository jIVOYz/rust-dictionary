use clap::Parser;

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    #[arg(short = 'w', long, required = true)]
    pub word: String,
    #[arg(short = 'm', long, num_args = 0..3, required = true)]
    pub meaning: Vec<String>,
    #[arg(short = 'e', long, num_args = 0..4, required = false)]
    pub example: Option<Vec<String>>,
}

#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    #[arg(required = true)]
    pub id: usize
}

#[derive(Parser, Debug)]
pub enum Cli {
    /// Add new word
    Add(AddArgs),
    /// Remove word
    Remove(RemoveArgs),
    /// Print all words
    List
}
