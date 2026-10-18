//! Isolation-flag coverage for `codetether mux new`.

use clap::Parser;

use crate::cli::command::mux_args::{MuxCommand, MuxStartOptions};
use crate::cli::{Cli, Command};

fn new_command(args: &[&str]) -> MuxCommand {
    let cli = Cli::try_parse_from(args).expect("mux new should parse");
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    args.command
}

#[test]
fn parses_no_worktree_fast_start() {
    assert!(matches!(
        new_command(&["codetether", "mux", "new", "work", "--no-worktree", "-d"]),
        MuxCommand::New {
            start: MuxStartOptions {
                no_worktree: true,
                detached: true
            },
            ..
        }
    ));
}

#[test]
fn defaults_to_managed_worktree_isolation() {
    assert!(matches!(
        new_command(&["codetether", "mux", "new", "work"]),
        MuxCommand::New {
            start: MuxStartOptions {
                no_worktree: false,
                detached: false
            },
            ..
        }
    ));
}

#[test]
fn help_advertises_no_worktree() {
    let help = Cli::try_parse_from(["codetether", "mux", "new", "--help"])
        .unwrap_err()
        .to_string();
    assert!(help.contains("--no-worktree"), "{help}");
}
