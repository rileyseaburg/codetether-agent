use super::Cli;
use clap::{CommandFactory, Parser};

#[test]
fn root_command_retains_application_package_identity() {
    let command = Cli::command();
    assert_eq!(command.get_version(), Some(env!("CARGO_PKG_VERSION")));
    assert_eq!(
        command.get_about().map(ToString::to_string),
        Some(env!("CARGO_PKG_DESCRIPTION").to_string())
    );
}

#[test]
fn bare_invocation_defaults_to_tui() {
    assert!(Cli::parse_from(["codetether"]).command.is_none());
}

#[path = "vault_help_tests.rs"]
mod vault_help;

#[test]
fn bare_invocation_accepts_yolo() {
    let cli = Cli::try_parse_from(["codetether", "--yolo"]).unwrap();
    assert!(cli.yolo);
    assert!(cli.command.is_none());
}
