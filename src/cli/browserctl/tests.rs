mod output;

use super::{BrowserCtlArgs, BrowserCtlCommand, request};
use crate::cli::{Cli, Command};
use clap::Parser;

fn parse_browserctl(args: &[&str]) -> BrowserCtlArgs {
    let cli = Cli::parse_from(std::iter::once("codetether").chain(args.iter().copied()));
    let Some(Command::Browserctl(args)) = cli.command else {
        panic!("expected browserctl command");
    };
    args
}

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
    let args = parse_browserctl(&["browserctl", "--ws-url", "http://127.0.0.1:9222", "list"]);
    assert!(request::needs_attach(&args.command));
    assert_eq!(request::attach(&args)["action"], "start");
    assert_eq!(request::command(&args)["action"], "tabs");
    assert_eq!(request::attach(&args)["ws_url"], "http://127.0.0.1:9222");
    assert_eq!(request::attach(&args)["headless"], true);
}

#[test]
fn global_headless_false_applies_to_attach_and_start() {
    let args = parse_browserctl(&["browserctl", "--headless", "false", "start"]);
    assert!(!args.headless);
    assert_eq!(request::attach(&args)["headless"], false);
    assert_eq!(request::command(&args)["headless"], false);
}

#[test]
fn health_command_does_not_attach() {
    let args = parse_browserctl(&["browserctl", "health"]);
    assert!(!request::needs_attach(&args.command));
    assert_eq!(request::command(&args)["action"], "health");
}
