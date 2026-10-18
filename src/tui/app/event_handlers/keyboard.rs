//! Native adapter for shared modifier shortcut selection.
use crate::tui::app::{session_runtime::TuiSessionHandle, state::App};
use crossterm::event::KeyEvent;
use std::path::Path;
mod host;
pub(super) fn handle_ctrl_key(
    app: &mut App,
    cwd: &Path,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> Option<anyhow::Result<bool>> {
    codetether_tui_navigation::modified::dispatch(&mut host::Native { app, cwd, runtime }, key)
}
#[cfg(not(test))]
fn handle_voice_key(app: &mut App) {
    super::voice::handle_voice_input(app);
}

#[cfg(test)]
fn handle_voice_key(app: &mut App) {
    app.state.status = "Voice shortcut".to_string();
}
