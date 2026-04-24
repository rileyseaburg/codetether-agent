//! Clipboard bridge CLI commands.

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Clipboard bridge arguments.
#[derive(Parser, Debug)]
pub struct ClipboardArgs {
    /// Clipboard bridge operation to run.
    #[command(subcommand)]
    pub command: ClipboardCommand,
}

/// Clipboard bridge subcommands.
#[derive(Subcommand, Debug)]
pub enum ClipboardCommand {
    /// Copy the local clipboard image as text for pasting over SSH
    Image(ClipboardImageArgs),
}

/// Arguments for copying a clipboard image into pasteable text.
#[derive(Parser, Debug)]
pub struct ClipboardImageArgs {
    /// Print the data URL instead of copying it back to the clipboard
    #[arg(long)]
    pub print: bool,
}

/// Execute a clipboard bridge subcommand.
///
/// # Errors
///
/// Returns an error when the requested clipboard operation cannot read or
/// write the system clipboard.
pub fn execute(args: ClipboardArgs) -> Result<()> {
    match args.command {
        ClipboardCommand::Image(args) => image(args.print),
    }
}

fn image(print: bool) -> Result<()> {
    let image = crate::image_clipboard::capture_image()?;
    if print {
        println!("{}", image.data_url);
        return Ok(());
    }
    crate::image_clipboard::copy_text(&image.data_url)?;
    eprintln!("Copied image paste text. Paste it into the remote CodeTether TUI.");
    Ok(())
}
