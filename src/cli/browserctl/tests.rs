use super::{BrowserCtlCommand, request};
use crate::cli::{Cli, Command};
use clap::Parser;

#[test]
fn browser_alias_parses_to_browserctl_command() {
    let cli = Cli::parse_from(["codetether", "browser", "list"]);
    assert!(matches!(
        cli.command,
        Some(Command::Browserctl(args))
            if matches!(args.command, BrowserCtlCommand::List)
    ));
}

#[test]
fn list_command_attaches_then_maps_to_tabs_action() {
    let cli = Cli::parse_from([
        "codetether",
        "browserctl",
        "--ws-url",
        "http://127.0.0.1:9222",
        "list",
    ]);
    let Some(Command::Browserctl(args)) = cli.command else {
        panic!("expected browserctl command");
    };
    assert!(request::needs_attach(&args.command));
    assert_eq!(request::attach(&args)["action"], "start");
    assert_eq!(request::command(&args)["action"], "tabs");
    assert_eq!(request::attach(&args)["ws_url"], "http://127.0.0.1:9222");
}
