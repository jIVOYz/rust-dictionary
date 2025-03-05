use clap::Parser;

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    #[arg(required = true)]
    pub name: String,
    #[arg(short = 'd', long, num_args = 0..3, required = true)]
    pub definition: Vec<String>,
    #[arg(short = 'e', long, num_args = 0..4, required = false)]
    pub example: Option<Vec<String>>,
}

#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    #[arg(required = true)]
    pub id: Vec<usize>,
}

#[derive(clap::Args, Debug)]
pub struct ListArgs {
    #[arg(long = "full", short = 'f', required = false)]
    /// Show words with examples
    pub full: bool,
    /// Show last N words
    #[arg(long = "last", short = 'l', required = false)]
    pub last: Option<usize>,
}

#[derive(clap::Args, Debug)]
pub struct SearchArgs {
    #[arg(required = true)]
    pub query: String,
}

#[derive(Parser, Debug)]
pub enum Cli {
    /// Add new word
    Add(AddArgs),
    /// Remove word
    Remove(RemoveArgs),
    /// Print all words
    List(ListArgs),
    /// Search for a word
    Search(SearchArgs),
}
