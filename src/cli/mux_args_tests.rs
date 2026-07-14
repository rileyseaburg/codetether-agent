use clap::Parser;

use crate::cli::command::mux_args::MuxCommand;
use crate::cli::{Cli, Command};

#[test]
fn parses_detached_named_mux_session() {
    let cli = Cli::try_parse_from(["codetether", "mux", "new", "-s", "work", "-d"])
        .expect("mux command should parse");
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    assert!(matches!(
        args.command,
        MuxCommand::New { session, detached: true, .. } if session == "work"
    ));
}

#[test]
fn parses_tmux_style_ls_alias() {
    let cli = Cli::try_parse_from(["codetether", "mux", "ls"]).expect("mux ls alias should parse");
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    assert!(matches!(args.command, MuxCommand::List { json: false }));
}
