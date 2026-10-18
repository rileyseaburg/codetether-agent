//! Native adapter for shared mode-specific input routing.
use crate::tui::app::{session_runtime::SessionSlot, state::App};
use crossterm::event::KeyEvent;
mod host;
pub(super) async fn handle_char_or_mode_key(app: &mut App, slot: &mut SessionSlot, key: KeyEvent) {
    codetether_tui_navigation::modes::dispatch(&mut host::Native { app, slot }, key).await;
}
