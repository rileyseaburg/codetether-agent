use clap::Parser;

use crate::cli::command::mux_args::{MuxCommand, MuxStartOptions};
use crate::cli::{Cli, Command};

#[test]
fn parses_mux_session_without_worktree() {
    let cli = Cli::try_parse_from([
        "codetether",
        "mux",
        "new",
        "shared",
        "/tmp",
        "--no-worktree",
    ])
    .unwrap();
    let Some(Command::Mux(args)) = cli.command else {
        panic!("expected mux command");
    };
    assert!(matches!(
        args.command,
        MuxCommand::New {
            start: MuxStartOptions {
                no_worktree: true,
                ..
            },
            ..
        }
    ));
}
