//! Synchronous editor action dispatch and buffer lifecycle.
use super::KeyHost;
use crate::{EditorInput, apply::apply, map_key};
use crossterm::event::KeyEvent;
use std::path::Path;

/// Handles a key while in editor mode. Returns `true` if it was consumed.
/// # Arguments
/// * `host` — Editor buffer, mode and frontend action bindings.
/// * `cwd` — Workspace directory used when opening the file finder.
/// * `key` — Incoming key event.
/// # Returns
/// Whether the key was consumed by editor handling.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_editor::interactive::KeyHost, key: crossterm::event::KeyEvent) {
/// let handled = codetether_editor::interactive::handle_editor_key(host, std::path::Path::new("."), key);
/// # }
/// ```
pub fn handle_editor_key(host: &mut impl KeyHost, cwd: &Path, key: KeyEvent) -> bool {
    if !host.in_editor() {
        return false;
    }
    let Some(action) = map_key(key) else {
        return true;
    };
    if host.approval_editing() && host.handle_approval_action(&action) {
        return true;
    }
    if action == EditorInput::OpenFinder {
        open_finder_from_editor(host, cwd);
        return true;
    }
    if host.approval_editing() && host.handle_approval_action(&action) {
        return true;
    }
    let Some(buf) = host.editor().as_mut() else {
        host.show_chat();
        return true;
    };
    match apply(buf, action) {
        Ok(true) => {}
        Ok(false) => {
            *host.editor() = None;
            host.reset_scroll();
            host.show_chat();
        }
        Err(e) => host.set_status(format!("editor save failed: {e}")),
    }
    host.redraw();
    true
}

/// Opens the fuzzy file finder without leaving the editor, warning if the
/// current buffer has unsaved changes (the edit is preserved until switch).
fn open_finder_from_editor(host: &mut impl KeyHost, cwd: &Path) {
    let dirty = host.editor().as_ref().is_some_and(|b| b.is_dirty());
    host.open_finder(cwd);
    if dirty {
        host.set_status(
            "Unsaved changes — Ctrl+S to save first, or pick a file to switch".to_string(),
        );
    }
    host.redraw();
}
