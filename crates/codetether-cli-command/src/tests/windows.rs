//! The installer health check must not invoke a model or require provider auth.

use super::{Cli, Command};
use clap::Parser;
use codetether_cli_options::{WindowsArgs, WindowsCommand};

#[test]
fn windows_ocr_probe_has_a_model_free_cli() {
    let cli =
        Cli::try_parse_from(["codetether", "windows", "ocr-status", "--require-ready"]).unwrap();
    assert!(matches!(
        cli.command,
        Some(Command::Windows(WindowsArgs {
            command: WindowsCommand::OcrStatus {
                require_ready: true
            }
        }))
    ));
}
