//! Isolation and validation coverage for `codetether mux resume`.

use clap::Parser;

use super::shared::parse;
use crate::cli::Cli;
use crate::cli::command::mux_args::{MuxCommand, MuxStartOptions};

#[test]
fn resume_accepts_no_worktree() {
    let command = parse(&[
        "codetether",
        "mux",
        "resume",
        "--session",
        "abc",
        "--no-worktree",
    ]);
    assert!(matches!(
        command,
        MuxCommand::Resume {
            start: MuxStartOptions {
                no_worktree: true,
                detached: false
            },
            ..
        }
    ));
}

#[test]
fn requires_a_session_id() {
    assert!(Cli::try_parse_from(["codetether", "mux", "resume"]).is_err());
}

#[test]
fn help_advertises_the_session_and_no_worktree_flags() {
    let help = Cli::try_parse_from(["codetether", "mux", "resume", "--help"])
        .unwrap_err()
        .to_string();
    assert!(help.contains("--session"), "{help}");
    assert!(help.contains("--no-worktree"), "{help}");
}
