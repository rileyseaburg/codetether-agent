//! Native bindings for shared chat scroll shortcuts.
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
mod host;
pub(super) fn handle_alt_scroll(app: &mut App, key: KeyEvent) -> Option<anyhow::Result<bool>> {
    codetether_tui_navigation::modified::scroll::dispatch(&mut host::Native(app), key).map(Ok)
}
