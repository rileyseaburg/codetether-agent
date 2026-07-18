use clap::Parser;

use crate::cli::command::mux_args::MuxCommand;
use crate::cli::{Cli, Command};

#[test]
fn parses_positional_kill_target() {
    let cli = Cli::try_parse_from(["codetether", "mux", "kill", "work"]).unwrap();
    let Some(Command::Mux(args)) = cli.command else {
        panic!()
    };
    assert!(matches!(args.command, MuxCommand::Kill {
        target: Some(target), named_target: None,
    } if target == "work"));
}

#[test]
fn parses_named_kill_target() {
    let cli = Cli::try_parse_from(["codetether", "mux", "kill", "--target", "work"]).unwrap();
    let Some(Command::Mux(args)) = cli.command else {
        panic!()
    };
    assert!(matches!(args.command, MuxCommand::Kill {
        target: None, named_target: Some(target),
    } if target == "work"));
}

#[test]
fn parses_kill_all_and_tmux_alias() {
    for command in ["kill-all", "kill-server"] {
        let cli = Cli::try_parse_from(["codetether", "mux", command]).unwrap();
        let Some(Command::Mux(args)) = cli.command else {
            panic!()
        };
        assert!(matches!(args.command, MuxCommand::KillAll));
    }
}
