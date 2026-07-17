use super::Cli;
use clap::Parser;

#[test]
fn bare_invocation_defaults_to_tui() {
    assert!(Cli::parse_from(["codetether"]).command.is_none());
}

#[test]
fn bare_invocation_accepts_yolo() {
    let cli = Cli::try_parse_from(["codetether", "--yolo"]).unwrap();
    assert!(cli.yolo);
    assert!(cli.command.is_none());
}
