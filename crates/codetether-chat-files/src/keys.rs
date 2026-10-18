//! Ctrl-key dispatch for shared-file actions in the Chat view.

use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};

use crate::Host;

use super::actions::{goto_shared_file_definition, show_shared_file_diff};

/// Fall-through Ctrl-key handler for shared-file actions.
///
/// `Ctrl+E` shows the shared file's git diff; `Ctrl+F` opens it in the file
/// viewer. Returns `None` for unhandled keys so the caller's match continues.
///
/// # Arguments
/// * `host` — Frontend state and file operations.
/// * `cwd` — Workspace directory for file references and diffs.
/// * `key` — Incoming terminal key event.
/// * `ctrl` — Whether the caller detected the Control modifier.
/// # Returns
/// `Some(Ok(false))` for a handled shortcut, otherwise `None`.
/// # Errors
/// This handler currently never returns an error; the result matches its caller.
/// # Examples
/// ```rust,no_run
/// fn handle(host: &mut impl codetether_chat_files::Host, key: crossterm::event::KeyEvent) {
///     let _ = codetether_chat_files::ctrl_key(host, std::path::Path::new("."), key, true);
/// }
/// ```
pub fn ctrl_key(
    host: &mut impl Host,
    cwd: &Path,
    key: KeyEvent,
    ctrl: bool,
) -> Option<anyhow::Result<bool>> {
    if !ctrl || !host.in_chat() {
        return None;
    }
    match key.code {
        KeyCode::Char('e') => show_shared_file_diff(host, cwd),
        KeyCode::Char('f') => goto_shared_file_definition(host, cwd),
        _ => return None,
    }
    Some(Ok(false))
}
