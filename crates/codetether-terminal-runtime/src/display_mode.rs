//! Symmetric terminal display-mode transitions for the TUI.
//!
//! Entering disables line wrapping so a full-width status row cannot scroll
//! the alternate screen. Leaving restores every mode changed during entry.

use std::io::Write;

use crossterm::{
    cursor::Show,
    event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste},
    execute,
    terminal::{DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
};

/// Enter the alternate screen with paste support and line wrapping disabled.
///
/// # Arguments
///
/// * `writer` — Terminal output receiving the crossterm control sequences.
///
/// # Returns
///
/// An I/O error if any mode transition cannot be written.
///
/// # Errors
/// Returns the first terminal-write error.
///
/// # Examples
/// ```
/// let mut output = Vec::new();
/// codetether_terminal_runtime::enter_display_mode(&mut output).unwrap();
/// assert!(!output.is_empty());
/// ```
pub fn enter(writer: &mut impl Write) -> std::io::Result<()> {
    execute!(
        writer,
        EnterAlternateScreen,
        EnableBracketedPaste,
        DisableLineWrap
    )
}

/// Restore line wrapping and leave the alternate screen.
///
/// # Arguments
///
/// * `writer` — Terminal output receiving the crossterm control sequences.
///
/// # Returns
///
/// An I/O error if any mode transition cannot be written.
///
/// # Errors
/// Returns the first terminal-write error.
///
/// # Examples
/// ```
/// let mut output = Vec::new();
/// codetether_terminal_runtime::leave_display_mode(&mut output).unwrap();
/// assert!(!output.is_empty());
/// ```
pub fn leave(writer: &mut impl Write) -> std::io::Result<()> {
    execute!(
        writer,
        Show,
        EnableLineWrap,
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableBracketedPaste
    )
}

#[cfg(test)]
#[path = "display_mode_tests.rs"]
mod tests;
