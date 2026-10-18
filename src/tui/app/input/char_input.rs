//! Native adapter for shared character routing.
use crate::tui::app::state::App;
use crossterm::event::KeyModifiers;
pub async fn handle_char(app: &mut App, modifiers: KeyModifiers, c: char) {
    codetether_tui_navigation::text::character(&mut super::text_host::Native(app), modifiers, c);
}
