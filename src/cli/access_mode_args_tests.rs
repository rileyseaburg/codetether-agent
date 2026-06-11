use super::{Cli, Command};
use crate::config::AccessMode;
use clap::Parser;

#[test]
fn run_accepts_access_mode_flag() {
    let cli = Cli::parse_from(["codetether", "run", "hello", "--access-mode", "approve"]);

    match cli.command {
        Some(Command::Run(args)) => assert_eq!(args.access_mode, Some(AccessMode::Approve)),
        other => panic!("expected run command, got {other:?}"),
    }
}

#[test]
fn tui_accepts_access_mode_flag() {
    let cli = Cli::parse_from(["codetether", "tui", "--access-mode", "full"]);

    match cli.command {
        Some(Command::Tui(args)) => assert_eq!(args.access_mode, Some(AccessMode::Full)),
        other => panic!("expected tui command, got {other:?}"),
    }
}
