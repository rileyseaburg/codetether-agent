//! Keyboard controls for an active Tether Catch play break.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::state::App;

pub(super) fn handle(app: &mut App, key: KeyEvent) -> bool {
    if key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
    {
        return false;
    }
    let Some(game) = app.state.interlude.as_mut() else {
        return false;
    };
    match key.code {
        KeyCode::Left | KeyCode::Char('a') => game.left(),
        KeyCode::Right | KeyCode::Char('d') => game.right(),
        _ => return false,
    }
    app.state.needs_redraw = true;
    true
}
