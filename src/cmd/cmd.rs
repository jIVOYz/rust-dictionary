use clap::Parser;

#[derive(Parser, Debug)]
pub enum Cmd {
    /// Add new word
    Add(Add),
    /// Remove word
    Remove(Remove),
    /// Edit word
    Edit(Edit),
    /// Print all words
    List(List),
    /// Search for a word
    Search(Search),
}

#[derive(clap::Args, Debug)]
pub struct Add {
    #[arg(required = true)]
    pub name: String,
    #[arg(short = 'd', long, num_args = 0..3, required = true)]
    pub definition: Vec<String>,
    #[arg(short = 'e', long, num_args = 0..4, required = false)]
    pub example: Option<Vec<String>>,
}

#[derive(clap::Args, Debug)]
pub struct Remove {
    #[arg(required = true)]
    pub id: Vec<usize>,
}

#[derive(clap::Args, Debug)]
pub struct Edit {
    #[arg(required = true)]
    pub id: usize,
}

#[derive(clap::Args, Debug)]
pub struct List {
    #[arg(long = "full", short = 'f', required = false)]
    /// Show words with examples
    pub full: bool,
    /// Show last N words
    #[arg(long = "last", short = 'l', required = false)]
    pub last: Option<usize>,
}

#[derive(clap::Args, Debug)]
pub struct Search {
    #[arg(required = true)]
    pub query: String,
}
