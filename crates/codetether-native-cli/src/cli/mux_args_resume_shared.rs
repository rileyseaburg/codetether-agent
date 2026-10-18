//! Shared parse helper for `codetether mux resume` tests.

use clap::Parser;

use crate::cli::command::mux_args::MuxCommand;
use crate::cli::{Cli, Command};

pub(super) fn parse(args: &[&str]) -> MuxCommand {
    let cli = Cli::try_parse_from(args).expect("mux resume should parse");
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    args.command
}
