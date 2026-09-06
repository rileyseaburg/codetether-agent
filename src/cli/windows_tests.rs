//! The installer health check must not invoke a model or require provider auth.

use super::*;
use crate::cli::{Cli, Command};
use clap::Parser;

#[test]
fn windows_ocr_probe_has_a_model_free_cli() {
    let cli = Cli::try_parse_from(["codetether", "windows", "ocr-status", "--require-ready"]).unwrap();
    assert!(matches!(cli.command, Some(Command::Windows(WindowsArgs {
        command: WindowsCommand::OcrStatus { require_ready: true }
    }))));
}

#[tokio::test]
#[cfg(not(windows))]
async fn windows_ocr_probe_does_not_claim_linux_readiness() {
    let error = run(WindowsArgs { command: WindowsCommand::OcrStatus {
        require_ready: true,
    }}).await.unwrap_err();
    assert!(error.to_string().contains("query failed"));
}