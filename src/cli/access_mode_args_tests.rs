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

#[test]
fn run_accepts_yolo_flag() {
    let cli = Cli::parse_from(["codetether", "run", "hello", "--yolo"]);

    match cli.command {
        Some(Command::Run(args)) => assert!(args.yolo),
        other => panic!("expected run command, got {other:?}"),
    }
}

#[test]
fn tui_accepts_yolo_flag() {
    let cli = Cli::parse_from(["codetether", "tui", "--yolo"]);

    match cli.command {
        Some(Command::Tui(args)) => assert!(args.yolo),
        other => panic!("expected tui command, got {other:?}"),
    }
}

#[test]
fn run_yolo_resolves_to_full_access_mode() {
    use super::run_config::effective_access_mode;
    let cli = Cli::parse_from(["codetether", "run", "hello", "--yolo"]);

    match cli.command {
        Some(Command::Run(args)) => {
            assert_eq!(effective_access_mode(&args), Some(AccessMode::Full));
        }
        other => panic!("expected run command, got {other:?}"),
    }
}
