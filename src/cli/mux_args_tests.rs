use clap::Parser;

use crate::cli::command::mux_args::MuxCommand;
use crate::cli::{Cli, Command};

#[test]
fn parses_detached_named_mux_session() {
    let cli = Cli::try_parse_from(["codetether", "mux", "new", "work", "/tmp", "-d"])
        .expect("mux command should parse");
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    assert!(matches!(
        args.command,
        MuxCommand::New { session, directory: Some(directory), detached: true }
            if session == "work" && directory == std::path::Path::new("/tmp")
    ));
}

#[test]
fn parses_positional_mux_attach() {
    let cli = Cli::try_parse_from(["codetether", "mux", "attach", "work"]).unwrap();
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    assert!(matches!(args.command, MuxCommand::Attach {
        target,
    } if target == "work"));
}

#[test]
fn parses_tmux_style_ls_alias() {
    let cli = Cli::try_parse_from(["codetether", "mux", "ls"]).expect("mux ls alias should parse");
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    assert!(matches!(args.command, MuxCommand::List { json: false }));
}
