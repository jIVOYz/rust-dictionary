mod add;
mod cmd;
mod edit;
mod list;
mod remove;
mod search;

pub use anyhow::Result;
pub use cmd::*;

pub trait Run {
    fn run(self: Self) -> Result<()>;
}

impl Run for Cmd {
    fn run(self: Self) -> Result<()> {
        match self {
            Cmd::Add(cmd) => cmd.run(),
            Cmd::Remove(cmd) => cmd.run(),
            Cmd::List(cmd) => cmd.run(),
            Cmd::Search(cmd) => cmd.run(),
            Cmd::Edit(cmd) => cmd.run(),
        }
    }
}
