//! Application redraw adapter for Tether Catch keys.
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
pub(super) fn handle(app: &mut App, key: KeyEvent) -> bool {
    let Some(game) = app.state.interlude.as_mut() else {
        return false;
    };
    if !codetether_interlude::handle_key(game, key) {
        return false;
    }
    app.state.needs_redraw = true;
    true
}
