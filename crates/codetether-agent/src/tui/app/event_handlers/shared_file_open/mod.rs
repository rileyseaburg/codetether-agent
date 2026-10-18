//! Chat-file shortcut binding and application regression coverage.
pub(super) mod host;
#[cfg(test)]
mod resolve;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_negative;
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
use std::path::Path;
pub(crate) fn ctrl_key(
    app: &mut App,
    cwd: &Path,
    key: KeyEvent,
    ctrl: bool,
) -> Option<anyhow::Result<bool>> {
    codetether_chat_files::ctrl_key(&mut host::Native(app), cwd, key, ctrl)
}
