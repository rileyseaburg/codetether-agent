//! Parsing regression tests for `codetether mux resume`.

use clap::Parser;

use crate::cli::command::mux_args::MuxCommand;
use crate::cli::{Cli, Command};

fn parse(args: &[&str]) -> MuxCommand {
    let cli = Cli::try_parse_from(args).expect("mux resume should parse");
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    args.command
}

#[test]
fn parses_session_id_flag() {
    let command = parse(&["codetether", "mux", "resume", "--session", "bd4e55f2"]);
    assert!(matches!(
        command,
        MuxCommand::Resume { session, name: None, detached: false }
            if session == "bd4e55f2"
    ));
}

#[test]
fn accepts_explicit_name_and_detached() {
    let command = parse(&[
        "codetether",
        "mux",
        "resume",
        "--session",
        "abc",
        "--name",
        "work",
        "-d",
    ]);
    assert!(matches!(
        command,
        MuxCommand::Resume { session, name: Some(name), detached: true }
            if session == "abc" && name == "work"
    ));
}

#[test]
fn requires_a_session_id() {
    assert!(Cli::try_parse_from(["codetether", "mux", "resume"]).is_err());
}

#[test]
fn help_advertises_the_session_flag() {
    let help = Cli::try_parse_from(["codetether", "mux", "resume", "--help"])
        .unwrap_err()
        .to_string();
    assert!(help.contains("--session"), "{help}");
}
