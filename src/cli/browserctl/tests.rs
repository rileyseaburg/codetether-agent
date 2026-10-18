//! Full CLI coverage of the browser command alias.
use crate::cli::{Cli, Command};
use super::{BrowserCtlArgs, BrowserCtlCommand};
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
