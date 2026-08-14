//! Parsing regression tests for `codetether mux resume`.

use crate::cli::command::mux_args::{MuxCommand, MuxStartOptions};

#[path = "mux_args_resume_isolation_tests.rs"]
mod isolation;
#[path = "mux_args_resume_shared.rs"]
mod shared;

use shared::parse;

#[test]
fn parses_session_id_flag() {
    let command = parse(&["codetether", "mux", "resume", "--session", "bd4e55f2"]);
    assert!(matches!(
        command,
        MuxCommand::Resume {
            session,
            name: None,
            start: MuxStartOptions { detached: false, no_worktree: false },
        }
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
        MuxCommand::Resume {
            session,
            name: Some(name),
            start: MuxStartOptions { detached: true, no_worktree: false },
        }
            if session == "abc" && name == "work"
    ));
}
