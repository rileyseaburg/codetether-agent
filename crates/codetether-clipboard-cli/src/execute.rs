//! Dispatch the clipboard operation selected by the CLI.
use crate::{ClipboardArgs, ClipboardCommand};
use anyhow::Result;

/// Execute a clipboard bridge subcommand.
///
/// # Arguments
/// * `args` - Clipboard command and output selection.
/// # Returns
/// Unit after printing or copying image text.
/// # Examples
/// ```no_run
/// use codetether_clipboard_cli::{execute, ClipboardArgs, ClipboardCommand, ClipboardImageArgs};
/// execute(ClipboardArgs { command: ClipboardCommand::Image(ClipboardImageArgs { print: true }) })?;
/// # Ok::<(), anyhow::Error>(())
/// ```
/// # Errors
///
/// Returns an error when the requested clipboard operation cannot read or
/// write the system clipboard.
pub fn execute(args: ClipboardArgs) -> Result<()> {
    match args.command {
        ClipboardCommand::Image(args) => crate::image::image(args.print),
    }
}
