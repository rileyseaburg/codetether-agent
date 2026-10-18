//! Windows diagnostic CLI schema.
use crate::WindowsCommand;
use clap::Parser;

/// Arguments for native Windows diagnostics.
///
/// # Examples
/// ```rust
/// use clap::Parser;
/// use codetether_cli_options::{WindowsArgs, WindowsCommand};
/// let args = WindowsArgs::parse_from(["windows", "ocr-status"]);
/// assert!(matches!(args.command, WindowsCommand::OcrStatus { require_ready: false }));
/// ```
#[derive(Clone, Debug, Parser)]
pub struct WindowsArgs {
    /// Requested diagnostic operation.
    #[command(subcommand)]
    pub command: WindowsCommand,
}
