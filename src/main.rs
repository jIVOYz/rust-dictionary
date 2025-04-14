mod cmd;
mod config;
use std::process::ExitCode;

use clap::Parser;
use cmd::{Cmd, Run};

fn main() {
    match Cmd::parse().run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("di: {err}");
            ExitCode::FAILURE
        }
    };
}
