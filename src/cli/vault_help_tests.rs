//! Help is a model-free, cross-platform login workflow, not just flag names.
use crate::cli::Cli;
use clap::{CommandFactory, Parser, error::ErrorKind};

#[test]
fn root_and_vault_help_explain_token_scope_and_the_login_workflow() {
    for args in [
        vec!["codetether", "--help"],
        vec!["codetether", "vault", "--help"],
        vec!["codetether", "vault", "login", "--help"],
        vec!["codetether", "vault", "login", "token", "--help"],
    ] {
        let error = Cli::try_parse_from(args).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
        let help = error.to_string();
        for expected in [
            "NOT Vault",
            "codetether vault url",
            "codetether vault login token",
            "codetether vault status",
            "codetether models",
            "CODETETHER_VAULT_SOURCE=env",
        ] {
            assert!(help.contains(expected), "missing {expected}: {help}");
        }
    }
}

#[test]
fn root_token_retains_control_plane_binding_without_showing_env_values() {
    let cli = Cli::command();
    let token = cli
        .get_arguments()
        .find(|arg| arg.get_id() == "token")
        .unwrap();
    assert_eq!(token.get_short(), Some('t'));
    assert_eq!(
        token.get_env(),
        Some(std::ffi::OsStr::new("CODETETHER_TOKEN"))
    );
    assert!(token.is_hide_env_values_set());
    assert!(
        token
            .get_help()
            .unwrap()
            .to_string()
            .contains("NOT a Vault token")
    );
}
