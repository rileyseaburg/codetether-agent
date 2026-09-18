//! CLI contracts do not require a Vault server, model, or saved credential.

use crate::cli::{Cli, Command};
use clap::Parser;

#[test]
fn vault_management_commands_parse_without_secret_arguments() {
    for args in [
        vec!["codetether", "vault"],
        vec!["codetether", "vault", "status"],
        vec!["codetether", "vault", "url", "https://vault.example"],
        vec!["codetether", "vault", "login", "token", "--stdin"],
        vec!["codetether", "vault", "login", "oidc", "--no-browser"],
        vec![
            "codetether",
            "vault",
            "login",
            "device",
            "--issuer",
            "https://issuer.example",
            "--client-id",
            "public-client",
        ],
        vec!["codetether", "vault", "logout"],
    ] {
        assert!(matches!(
            Cli::try_parse_from(args).unwrap().command,
            Some(Command::Vault(_))
        ));
    }
    assert!(
        Cli::try_parse_from(["codetether", "vault", "login", "token", "fixture-secret"]).is_err()
    );
}
