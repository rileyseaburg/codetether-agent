//! Editor and agent argv must parse identically through the actual root CLI.
use super::{Cli, Command};
use clap::Parser;
use codetether_rust_build::RustAction;

#[test]
fn rust_proposal_is_model_only_without_execution_or_approval_flags() {
    let cli =
        Cli::try_parse_from(["codetether", "rust-propose", "--model", "provider/model"]).unwrap();
    let Some(Command::RustPropose(args)) = cli.command else {
        panic!("expected RustPropose command")
    };
    assert_eq!(args.model.as_deref(), Some("provider/model"));
    for argument in ["--apply", "--approve", "--session"] {
        assert!(Cli::try_parse_from(["codetether", "rust-propose", argument]).is_err());
    }
}

#[test]
fn rust_build_accepts_editor_options_before_action() {
    let cli = Cli::try_parse_from([
        "codetether",
        "rust",
        "--cwd",
        "/repo/work tree",
        "--toolchain",
        "stable",
        "--package",
        "app",
        "--bin",
        "app",
        "build",
    ])
    .unwrap();
    let Some(Command::Rust(args)) = cli.command else {
        panic!("expected Rust command")
    };
    assert_eq!(args.action, RustAction::Build);
    assert_eq!(args.package.as_deref(), Some("app"));
    assert_eq!(args.cwd.to_str(), Some("/repo/work tree"));
}

#[test]
fn rust_build_preserves_runtime_arguments_and_rejects_global_enable() {
    let cli =
        Cli::try_parse_from(["codetether", "rust", "run", "--arg=--help", "--arg=a b"]).unwrap();
    let Some(Command::Rust(args)) = cli.command else {
        panic!("expected Rust command")
    };
    assert_eq!(args.args, ["--help", "a b"]);
    assert!(Cli::try_parse_from(["codetether", "rust", "enable"]).is_err());
}
