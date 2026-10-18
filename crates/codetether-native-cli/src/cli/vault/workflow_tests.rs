//! Failed native commands return a usable workflow alongside the original error.
use crate::cli::{Cli, Command};
use clap::Parser;

#[tokio::test]
async fn vault_command_failure_includes_next_steps_without_provider_startup() {
    let cli = Cli::try_parse_from(["codetether", "vault", "url", "not-a-url"]).unwrap();
    let Some(Command::Vault(args)) = cli.command else {
        panic!("Vault command expected")
    };
    let error = super::execute(args).await.unwrap_err();
    let report = format!("{error:#}");
    assert!(report.contains("codetether vault login token"));
    assert!(report.contains("codetether vault status"));
    assert!(report.contains("codetether models"));
    assert!(report.contains("-t/--token"));
    assert!(
        error.chain().count() > 1,
        "original failure must remain available"
    );
}
