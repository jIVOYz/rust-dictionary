mod cmd;
mod config;
use clap::Parser;
use cmd::{Cmd, Run};

fn main() {
    Cmd::parse().run();
}
