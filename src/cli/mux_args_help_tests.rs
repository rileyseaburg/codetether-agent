//! User-facing positional mux syntax regression tests.

use clap::Parser;

use crate::cli::Cli;

#[test]
fn help_advertises_positional_new_and_attach() {
    let new = Cli::try_parse_from(["codetether", "mux", "new", "--help"])
        .unwrap_err()
        .to_string();
    let attach = Cli::try_parse_from(["codetether", "mux", "attach", "--help"])
        .unwrap_err()
        .to_string();
    assert!(
        new.contains("codetether mux new [OPTIONS] <SESSION> [DIRECTORY]"),
        "{new}"
    );
    assert!(
        attach.contains("codetether mux attach [OPTIONS] <SESSION>"),
        "{attach}"
    );
    assert!(!new.contains("--session"));
    assert!(!attach.contains("--target"));
}
