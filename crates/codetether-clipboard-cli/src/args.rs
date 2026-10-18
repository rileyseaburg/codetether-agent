//! Clipboard command selection and output options.
use clap::{Parser, Subcommand};

/// Clipboard bridge arguments.
/// # Examples
/// ```
/// use codetether_clipboard_cli::{ClipboardArgs, ClipboardCommand, ClipboardImageArgs};
/// let args = ClipboardArgs { command: ClipboardCommand::Image(ClipboardImageArgs { print: true }) };
/// assert!(matches!(args.command, ClipboardCommand::Image(_)));
/// ```
#[derive(Parser, Debug)]
pub struct ClipboardArgs {
    /// Clipboard bridge operation to run.
    #[command(subcommand)]
    pub command: ClipboardCommand,
}

/// Clipboard bridge subcommands.
/// # Examples
/// ```
/// use codetether_clipboard_cli::{ClipboardCommand, ClipboardImageArgs};
/// match ClipboardCommand::Image(ClipboardImageArgs { print: false }) { ClipboardCommand::Image(args) => assert!(!args.print) }
/// ```
#[derive(Subcommand, Debug)]
pub enum ClipboardCommand {
    /// Copy the local clipboard image as text for pasting over SSH
    Image(ClipboardImageArgs),
}

/// Arguments for copying a clipboard image into pasteable text.
/// # Examples
/// ```
/// let args = codetether_clipboard_cli::ClipboardImageArgs { print: true };
/// assert!(args.print);
/// ```
#[derive(Parser, Debug)]
pub struct ClipboardImageArgs {
    /// Print the data URL instead of copying it back to the clipboard
    #[arg(long)]
    pub print: bool,
}
