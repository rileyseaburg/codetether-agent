//! Bind native editor keys to shared action and buffer lifecycle policy.
#[path = "approval_editor/mod.rs"]
mod approval_editor;
#[path = "editor_key_host.rs"]
mod host;
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
use std::path::Path;
pub(crate) fn handle_editor_key(app: &mut App, cwd: &Path, key: KeyEvent) -> bool {
    codetether_editor::interactive::handle_editor_key(&mut host::Native(app), cwd, key)
}
